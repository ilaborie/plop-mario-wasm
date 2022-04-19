use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, Event};

use crate::assets::audio::musics::{MusicPlayer, Track};

pub mod musics;
pub mod sounds;

pub struct MusicController {
    music_player: Rc<MusicPlayer>,
}

impl MusicController {
    pub fn new(music_player: Rc<MusicPlayer>) -> Self {
        Self { music_player }
    }

    pub fn pause(&self) {
        self.music_player.pause();
    }

    pub fn play_theme(&self) {
        let _ = self.music_player.play(Track::Main, 1.);
    }

    pub fn play_hurry(&self) {
        if let Some(audio) = self.music_player.clone().play(Track::Hurry, 1.) {
            let player = self.music_player.clone();
            let closure = Closure::wrap(Box::new(move |_: Event| {
                player.play(Track::Main, 1.3);
            }) as Box<dyn FnMut(_)>);

            let mut options = AddEventListenerOptions::new();
            options.once(true);

            audio
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
}
