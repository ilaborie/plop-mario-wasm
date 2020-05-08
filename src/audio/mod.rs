use crate::audio::musics::{MusicPlayer, Track};
use crate::utils::log;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, Event};

pub mod musics;
pub mod sounds;

pub struct MusicController {
    music_player: Rc<RefCell<MusicPlayer>>,
}

impl MusicController {
    pub fn new(music_player: MusicPlayer) -> Self {
        let music_player = Rc::new(RefCell::new(music_player));
        Self { music_player }
    }

    pub fn play_theme(&self) {
        let _ = self.music_player.borrow().play(Track::Main, 1.);
    }

    pub fn play_hurry(&self) {
        let player = self.music_player.clone();
        let closure = Closure::wrap(Box::new(move |_: Event| {
            log("Music End");
            player.borrow().play(Track::Main, 1.3);
        }) as Box<dyn FnMut(_)>);

        let mut options = AddEventListenerOptions::new();
        options.once(true);

        self.music_player
            .clone()
            .borrow()
            .play(Track::Hurry, 1.)
            .add_event_listener_with_callback_and_add_event_listener_options(
                "ended",
                closure.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap();

        // The instance of `Closure` that we created will invalidate its
        // corresponding JS callback whenever it is dropped, so if we were to
        // normally return from `setup_clock` then our registered closure will
        // raise an exception when invoked.
        //
        // Normally we'd store the handle to later get dropped at an appropriate
        // time but for now we want it to be a global handler so we use the
        // `forget` method to drop it without invalidating the closure. Note that
        // this is leaking memory in Rust, so this should be done judiciously!
        closure.forget();
    }
}
