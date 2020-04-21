use wasm_bindgen::prelude::*;
use crate::assets::Sprite;
use core::ops::Range;
use core::slice::Iter;

#[derive(Deserialize, Hash, Clone, Copy, PartialEq, Eq)]
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

#[derive(Deserialize, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Background {
    tile: Sprite,
    ranges: Ranges,
}

impl Background {
    pub fn tile(&self) -> Sprite {
        self.tile
    }
    pub fn ranges(&self) -> Ranges {
        self.ranges
    }
}

#[wasm_bindgen]
#[derive(Deserialize, Hash, Clone, PartialEq, Eq)]
pub struct Level {
    backgrounds: Vec<Background>,
}

#[wasm_bindgen]
impl Level {
    pub fn new(json: &JsValue) -> Self {
        json.into_serde::<Level>().unwrap()
    }

    pub(crate) fn backgrounds(&self) -> Iter<'_, Background> {
        self.backgrounds.iter()
    }
}
