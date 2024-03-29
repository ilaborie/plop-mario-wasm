use std::collections::HashMap;

use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlAudioElement, Request, Response};

use crate::utils::{log, window};

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Track {
    Main,
    Hurry,
}

#[derive(Deserialize)]
struct TrackDescription {
    url: String,
}

#[derive(Deserialize)]
struct MusicDescription {
    main: Option<TrackDescription>,
    hurry: Option<TrackDescription>,
}

impl MusicDescription {
    pub async fn load(name: &str) -> Result<MusicDescription, JsValue> {
        log(&format!("Loading music sheet '{}'", name));
        let url = format!("assets/musics/{}.json", name);
        let request = Request::new_with_str(&url)?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let music_description = json
            .into_serde::<MusicDescription>()
            .expect("Error during music loading");

        Ok(music_description)
    }
}

#[derive(Default)]
pub struct MusicPlayer {
    tracks: HashMap<Track, HtmlAudioElement>,
    volume: f64,
}

impl MusicPlayer {
    pub async fn load_music(name: &str, volume: f64) -> Result<MusicPlayer, JsValue> {
        let desc = MusicDescription::load(name).await?;

        let mut result = MusicPlayer {
            volume,
            ..Default::default()
        };

        if let Some(main) = desc.main {
            result.add_track(Track::Main, main.url.as_str(), true);
        }
        if let Some(hurry) = desc.hurry {
            result.add_track(Track::Hurry, hurry.url.as_str(), false);
        }
        Ok(result)
    }

    fn add_track(&mut self, track: Track, url: &str, looping: bool) {
        let audio = HtmlAudioElement::new_with_src(url).unwrap();
        audio.set_loop(looping);

        self.tracks.insert(track, audio);
    }

    pub fn pause(&self) {
        for audio in self.tracks.values() {
            audio.pause().unwrap();
        }
    }

    pub fn play(&self, track: Track, speed: f64) -> Option<&HtmlAudioElement> {
        self.pause();

        self.tracks.get(&track).map(|audio| {
            let _ = audio.play().unwrap();
            audio.set_volume(self.volume);
            audio.set_playback_rate(speed);
            audio
        })
    }
}
