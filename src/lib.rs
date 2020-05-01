use crate::assets::config::Configuration;
use crate::assets::TILE_SIZE;
use crate::physics::Size;
use crate::system::System;
use crate::utils::{body, canvas, context_2d, log, request_animation_frame, set_panic_hook, time};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod assets;
mod camera;
mod entity;
mod keyboard;
mod layers;
mod level;
mod physics;
pub mod system;
mod utils;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const DELTA_TIME: f64 = 1. / 60.;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_log!("Starting...");
    set_panic_hook();

    // Config
    let config = Configuration::load().await?;

    // Canvas
    let size = Size::new(
        TILE_SIZE * config.view.width,
        TILE_SIZE * config.view.height,
    );
    let can = canvas(size);
    body().append_child(&can)?;
    let context = context_2d(&can);

    // System / Player
    let mut sys = System::create(&config, "1-1", "mario").await?;

    // Timer
    let mut last_time = 0.0;
    let mut accumulated_time = 0.0;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // if last_time > 1_000. { // FIXME Stop after 1s
        //     log("Stop");
        //     let _ = f.borrow_mut().take();
        //     return;
        // }

        let time = time();
        accumulated_time += (time - last_time) / 1000.0;

        while accumulated_time > DELTA_TIME {
            sys.update(DELTA_TIME);
            sys.draw(&context);

            accumulated_time -= DELTA_TIME;
        }
        last_time = time;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
