use crate::keyboard::Action;
use crate::physics::motion::Direction;
use crate::physics::size::Size;
use crate::utils::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct JumpingDefault {
    pub duration: f64,
    pub velocity: f64,
    #[serde(alias = "gracePeriod")]
    pub grace_period: f64,
    #[serde(alias = "speedBoost")]
    pub speed_boost: f64,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct MotionDefault {
    pub direction: Direction,
    #[serde(alias = "acceleration")]
    pub acceleration_base: f64,
    #[serde(alias = "deceleration")]
    pub deceleration_base: f64,
    #[serde(alias = "dragFactor")]
    pub drag_factor: f64,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct PhysicsDefault {
    pub jumping: JumpingDefault,
    pub motion: MotionDefault,
    pub gravity: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    keymap: HashMap<String, Action>,
    pub view: Size,
    pub physics: PhysicsDefault,
}

impl Configuration {
    pub async fn load() -> Result<Configuration, JsValue> {
        let request = Request::new_with_str("/assets/config.json")?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let config = json
            .into_serde::<Configuration>()
            .expect("Error during level loading");

        Ok(config)
    }

    pub fn action(&self, key_code: String) -> Option<Action> {
        self.keymap.get(&key_code).map(|&action| action)
    }
}
