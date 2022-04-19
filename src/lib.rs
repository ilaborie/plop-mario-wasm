use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::assets::{Assets, TILE_SIZE};
use crate::physics::Size;
use crate::system::System;
use crate::utils::{body, canvas, context_2d, log, request_animation_frame, set_panic_hook, time};

pub mod assets;
mod camera;
mod entity;
mod events;
mod game;
mod input;
mod layers;
mod physics;
mod scene;
pub mod system;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const DELTA_TIME: f64 = 1. / 60.;

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    console_log!("Starting...");
    set_panic_hook();

    // assets
    let assets = Assets::load().await?;
    let config = assets.configuration();

    // Canvas
    let size = Size::new(
        TILE_SIZE * config.view.width,
        TILE_SIZE * config.view.height,
    );
    let can = canvas(size);
    body().append_child(&can)?;
    let context = Rc::new(context_2d(&can));

    // System / Player
    let mut sys = System::new(assets, context);
    sys.start("mario");

    timer(Box::new(move || sys.update(DELTA_TIME)));

    Ok(())
}

fn timer(mut updater: Box<dyn FnMut()>) {
    let mut last_time = 0.;
    let mut accumulated_time = 0.;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let time = time();

        if last_time > 0. {
            accumulated_time += (time - last_time) / 1000.;
            while accumulated_time > DELTA_TIME {
                updater();
                accumulated_time -= DELTA_TIME;
            }
        }
        last_time = time;

        // Check ending
        // if player.borrow().time().get() < 0. {
        //     log("Finished!");
        //     let _ = f.borrow_mut().take();
        //     return;
        // }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
