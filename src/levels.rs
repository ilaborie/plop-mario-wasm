use crate::assets::Sprite;
use wasm_bindgen::__rt::core::ops::Range;
use wasm_bindgen::__rt::core::slice::Iter;


#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Deserialize, Hash, Clone, Debug, PartialEq, Eq)]
pub struct Level {
    backgrounds: Vec<Background>,
}

impl Level {
    pub fn backgrounds(&self) -> Iter<'_, Background> {
        self.backgrounds.iter()
    }
}

