use std::collections::HashMap;

use js_sys::ArrayBuffer;
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioContext, Request, Response};

use crate::utils::{log, window};

#[derive(Deserialize)]
pub struct FxDescription {
    url: String,
}

#[derive(Deserialize, Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fx {
    #[serde(alias = "jump")]
    Jump,
    #[serde(alias = "stomp")]
    Stomp,
    #[serde(alias = "shoot")]
    Shoot,
    #[serde(alias = "coin")]
    Coin,
}

async fn load_audio_buffer(
    url: &str,
    audio_context: &AudioContext,
) -> Result<AudioBuffer, JsValue> {
    log(&format!("Loading audio file '{}'", url));
    let request = Request::new_with_str(url)?;

    let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let array_buffer = JsFuture::from(resp.array_buffer()?)
        .await?
        .dyn_into::<ArrayBuffer>()?;

    let audio_buffer = JsFuture::from(audio_context.decode_audio_data(&array_buffer).unwrap())
        .await?
        .dyn_into::<AudioBuffer>()?;

    Ok(audio_buffer)
}

#[derive(Deserialize)]
pub struct SoundAudioDescription {
    fx: HashMap<Fx, FxDescription>,
}

impl SoundAudioDescription {
    async fn load(name: &str) -> Result<SoundAudioDescription, JsValue> {
        log(&format!("Loading sound sheet '{}'", name));
        let url = format!("assets/sounds/{}.json", name);
        let request = Request::new_with_str(&url)?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let audio_description = json
            .into_serde::<SoundAudioDescription>()
            .expect("Error during sounds loading");

        Ok(audio_description)
    }
}

#[derive(Default)]
pub struct AudioBoard {
    map: HashMap<Fx, AudioBuffer>,
    volume: f32,
}

impl AudioBoard {
    pub async fn load_sounds(name: &str, volume: f32) -> Result<AudioBoard, JsValue> {
        let desc = SoundAudioDescription::load(name).await?;
        let audio_context = AudioContext::new().unwrap();

        let mut map = HashMap::new();
        for (fx, desc) in desc.fx {
            let audio = load_audio_buffer(desc.url.as_str(), &audio_context).await?;
            map.insert(fx, audio);
        }

        let result = Self { map, volume };
        Ok(result)
    }

    pub fn play(&self, audio_context: &AudioContext, fx: Fx) {
        let audio_buffer = self
            .map
            .get(&fx)
            .unwrap_or_else(|| panic!("No fx {:?} found", fx));

        // Volume
        let gain = audio_context.create_gain().unwrap();
        gain.gain().set_value(self.volume);
        gain.connect_with_audio_node(&audio_context.destination())
            .unwrap();

        let source = audio_context.create_buffer_source().unwrap();
        source.connect_with_audio_node(&gain).unwrap();
        source.set_buffer(Some(audio_buffer));

        // Play
        source.start().unwrap();
    }
}
