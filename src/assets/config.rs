use crate::assets::sprites::Rectangle;
use crate::keyboard::Action;
use crate::physics::motion::Direction;
use crate::physics::size::Size;
use crate::utils::window;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};
use crate::physics::position::Position;

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
pub struct PlayerDefault {
    pub position: Position,
    pub size: Size,
    pub jumping: JumpingDefault,
    pub motion: MotionDefault,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct MobsDefault {
    pub speed: f64,
    pub size: Size,
    pub bbox: Option<Rectangle>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DevConfiguration {
    #[serde(alias = "showCollision")]
    pub(crate) show_collision: bool,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub dev: DevConfiguration,
    keymap: HashMap<String, Action>,
    pub view: Size,
    pub gravity: f64,
    pub player: PlayerDefault,
    pub mobs: HashMap<String, MobsDefault>,
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
        self.keymap.get(&key_code).copied()
    }
}
