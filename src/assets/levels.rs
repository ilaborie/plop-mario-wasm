use crate::assets::sprites::Sprite;
use crate::utils::window;
use core::ops::Range;
use core::slice::Iter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

#[derive(Serialize, Deserialize)]
pub struct Ranges {
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
}

impl Ranges {
    pub fn x(&self) -> Range<u32> {
        self.x0..self.x1
    }
    pub fn y(&self) -> Range<u32> {
        self.y0..self.y1
    }
}

#[derive(Serialize, Deserialize)]
pub struct Background {
    tile: Sprite,
    ranges: Vec<Ranges>,
}

impl Background {
    pub fn tile(&self) -> Sprite {
        self.tile
    }
    pub fn ranges(&self) -> Iter<'_, Ranges> {
        self.ranges.iter()
    }
}

#[derive(Serialize, Deserialize)]
pub struct LevelDefinition {
    backgrounds: Vec<Background>,
}

impl LevelDefinition {
    pub(crate) fn backgrounds(&self) -> Iter<'_, Background> {
        self.backgrounds.iter()
    }
}

pub async fn load_level(name: &str) -> Result<LevelDefinition, JsValue> {
    let url = format!("/levels/{}.json", name);
    let request = Request::new_with_str(&url)?;

    let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let level = json.into_serde::<LevelDefinition>().unwrap();
    Ok(level)
}
