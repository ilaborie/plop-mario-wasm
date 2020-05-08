use crate::utils::{log, window};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlAudioElement, Request, Response};

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Track {
    Main,
    Hurry,
}

#[derive(Deserialize)]
struct MainDescription {
    url: String,
}

#[derive(Deserialize)]
struct MusicDescription {
    main: MainDescription,
    hurry: MainDescription,
}

impl MusicDescription {
    pub async fn load(name: &str) -> Result<MusicDescription, JsValue> {
        log(&format!("Loading music sheet '{}'", name));
        let url = format!("/assets/musics/{}.json", name);
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
}

impl MusicPlayer {
    pub async fn load_music(name: &str) -> Result<MusicPlayer, JsValue> {
        let desc = MusicDescription::load(name).await?;

        let mut result = Self::default();
        result.add_track(Track::Main, desc.main.url.as_str(), true);
        result.add_track(Track::Hurry, desc.hurry.url.as_str(), false);

        Ok(result)
    }

    fn add_track(&mut self, track: Track, url: &str, looping: bool) {
        let audio = HtmlAudioElement::new_with_src(url).unwrap();
        audio.set_loop(looping);
        audio.set_volume(0.5);

        self.tracks.insert(track, audio);
    }

    pub fn play(&self, track: Track, speed: f64) -> &HtmlAudioElement {
        for audio in self.tracks.values() {
            audio.pause().unwrap();
        }

        let audio: &HtmlAudioElement = self
            .tracks
            .get(&track)
            .unwrap_or_else(|| panic!("Music track {:?} not found!", track));
        let _ = audio.play().unwrap();
        audio.set_playback_rate(speed);

        audio
    }
}
