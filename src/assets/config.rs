use crate::assets::load_json;
use crate::assets::sprites::Rectangle;
use crate::input::Action;
use crate::physics::{Direction, Position, Size};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

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
    pub stomp: f64,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct MobsDefault {
    pub speed: f64,
    pub size: Size,
    pub bbox: Option<Rectangle>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct DevConfiguration {
    #[serde(alias = "showCollision")]
    pub(crate) show_collision: bool,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct SoundsConfiguration {
    pub fx: f32,
    pub music: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub dev: DevConfiguration,
    pub sounds: SoundsConfiguration,
    keymap: HashMap<String, Action>,
    pub view: Size,
    pub gravity: f64,
    pub player: PlayerDefault,
    pub mobs: HashMap<String, MobsDefault>,
}

impl Configuration {
    pub async fn load() -> Result<Configuration, JsValue> {
        let config = load_json("assets/config.json")
            .await?
            .into_serde::<Configuration>()
            .expect("Error during level loading");

        Ok(config)
    }

    pub fn keymap(&self) -> HashMap<String, Action> {
        self.keymap.clone()
    }

    pub fn action(&self, key_code: String) -> Option<Action> {
        self.keymap.get(&key_code).copied()
    }
}
