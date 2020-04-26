use crate::assets::config::Configuration;
use crate::assets::TILE_SIZE;
use crate::entity::Updatable;
use crate::system::System;
use crate::utils::{body, canvas, context_2d, log, request_animation_frame, set_panic_hook, time};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod assets;
mod camera;
mod entity;
mod keyboard;
mod layers;
mod level;
mod physics;
mod system;
mod utils;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_log!("Starting...");
    set_panic_hook();

    // Config
    let config = Configuration::load().await?;

    // Canvas
    let can = canvas(
        TILE_SIZE * config.view.width,
        TILE_SIZE * config.view.height,
    );
    body().append_child(&can)?;
    let context = context_2d(&can);

    // System
    let mut sys = System::create(config, "1-1", "mario").await?;
    sys.register_keyboard();
    // sys.register_mouse(&can);

    // Timer
    let delta_time = 1.0 / 60.0;
    let mut last_time = 0.0;
    let mut accumulated_time = 0.0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // if system.stopped() {
        //     let _ = f.borrow_mut().take();
        //     return;
        // }

        let time = time();
        accumulated_time += (time - last_time) / 1000.0;

        while accumulated_time > delta_time {
            // console_log!("dt {}", delta_time);
            sys.update(delta_time);
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
