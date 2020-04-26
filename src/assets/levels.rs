use crate::assets::animations::AnimationName;
use crate::assets::sprites::{Sprite, SpriteSheet};
use crate::assets::TILE_SIZE;
use crate::physics::matrix::Matrix;
use crate::physics::size::Size;
use crate::physics::tile_resolver::TileData;
use crate::utils::window;
use core::ops::Range;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    #[serde(alias = "ground")]
    Ground,
}

pub struct Ranges {
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
}

impl Ranges {
    pub fn new(values: &Vec<u32>) -> Self {
        let (x0, x1, y0, y1) = match values.len() {
            2 => {
                let x0 = values[0];
                let x1 = x0 + 1;
                let y0 = values[1];
                let y1 = y0 + 1;
                (x0, x1, y0, y1)
            }
            3 => {
                let x0 = values[0];
                let x1 = x0 + values[1];
                let y0 = values[2];
                let y1 = y0 + 1;
                (x0, x1, y0, y1)
            }
            4 => {
                let x0 = values[0];
                let x1 = x0 + values[1];
                let y0 = values[2];
                let y1 = y0 + values[3];
                (x0, x1, y0, y1)
            }
            _ => panic!("Invalid ranges {:?}, expected a size of 2,3,4", values),
        };

        Self { x0, x1, y0, y1 }
    }

    pub fn x(&self) -> Range<u32> {
        self.x0..self.x1
    }
    pub fn y(&self) -> Range<u32> {
        self.y0..self.y1
    }
}

#[derive(Serialize, Deserialize)]
pub struct Background {
    #[serde(alias = "sprite")]
    tile: Sprite,
    #[serde(alias = "type")]
    kind: Option<Kind>,
    animation: Option<AnimationName>,
    ranges: Vec<Vec<u32>>,
}

impl Background {
    pub fn tile(&self) -> Sprite {
        self.tile
    }
    pub fn kind(&self) -> Option<Kind> {
        self.kind
    }
    pub fn animation(&self) -> Option<AnimationName> {
        self.animation
    }
    pub fn ranges(&self) -> Vec<Ranges> {
        self.ranges.iter().map(|v| Ranges::new(v)).collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct LevelDefinition {
    #[serde(alias = "spriteSheet")]
    sprite_sheet: String,
    backgrounds: Vec<Background>,
    gravity: Option<f64>,
}

impl LevelDefinition {
    pub async fn load(name: &str) -> Result<LevelDefinition, JsValue> {
        // log(&format!("Loading level {}", name).to_string());
        let url = format!("/assets/levels/{}.json", name);
        let request = Request::new_with_str(&url)?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let level = json
            .into_serde::<LevelDefinition>()
            .expect("Error during level loading");

        Ok(level)
    }

    pub async fn build(&self) -> Result<(Matrix<TileData>, SpriteSheet, Option<f64>), JsValue> {
        // Size
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        for bg in self.backgrounds.iter() {
            for range in bg.ranges() {
                for x in range.x() {
                    for y in range.y() {
                        width = width.max(x);
                        height = height.max(y);
                    }
                }
            }
        }
        let size = Size::new(width + 1, height + 1);

        // Matrix
        let mut tiles = Matrix::new(size);
        for bg in self.backgrounds.iter() {
            for range in bg.ranges() {
                for x in range.x() {
                    for y in range.y() {
                        let left = (x * TILE_SIZE) as f64;
                        let top = (y * TILE_SIZE) as f64;
                        let data = TileData::new(
                            bg.tile(),
                            bg.kind(),
                            bg.animation(),
                            top,
                            left + TILE_SIZE as f64,
                            top + TILE_SIZE as f64,
                            left,
                        );
                        tiles.set(x as usize, y as usize, data);
                    }
                }
            }
        }

        let sprite_sheet = SpriteSheet::load(self.sprite_sheet.as_str()).await?;
        Ok((tiles, sprite_sheet, self.gravity))
    }
}
