use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::{ body, request_animation_frame, time, context_2d, canvas, set_panic_hook};
use crate::assets::{load_player_sprites, load_background_sprites};
use crate::system::System;
use crate::entity::player::PlayerEntity;
use crate::assets::levels::load_level;
use crate::assets::sprites::Sprite;

mod assets;
mod entity;
mod layers;
mod system;
mod utils;
mod keyboard;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_log!("Starting...");
    set_panic_hook();

    let delta_time = 1.0 / 60.0;
    let mut last_time = 0.0;
    let mut accumulated_time = 0.0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let can = canvas(640, 640);
    body().append_child(&can)?;

    // FIXME create System
    let context = context_2d(&can);
    let gravity= 3000.0;
    let level = load_level("lvl_1-1").await?;
    let bg_sprites = load_background_sprites().await?;
    let player_sprites = load_player_sprites().await?;

    let mut player_entity = PlayerEntity::new(Sprite::MarioIdle, player_sprites, gravity);
    player_entity.set_position(64.0, 180.0);
    player_entity.set_velocity(200.0, -600.0);

    let mut sys = System::new(level, bg_sprites, player_entity);
    sys.register_keyboard();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // if system.stopped() {
        //     let _ = f.borrow_mut().take();
        //     return;
        // }

        let time = time();
        accumulated_time += (time - last_time) / 1000.0;

        while accumulated_time > delta_time {
            // console_log!("dt {}", delta_time);
            sys.update_player(delta_time);
            sys.draw(&context);

            accumulated_time -= delta_time;
        }
        last_time = time;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
