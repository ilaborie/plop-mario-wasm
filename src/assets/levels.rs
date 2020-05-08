use crate::assets::sprites::{AnimationName, Sprite, SpriteSheet};
use crate::assets::TILE_SIZE;
use crate::physics::bounding_box::BBox;
use crate::physics::matrix::Matrix;
use crate::physics::{Position, Size};
use crate::utils::{log, window};
use core::slice::Iter;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    #[serde(alias = "ground")]
    Ground,
    #[serde(alias = "brick")]
    Brick,
    #[serde(alias = "brick-broken")]
    BrickBroken,
    #[serde(alias = "coin")]
    Coin,
}

pub struct Ranges {
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
}

impl Ranges {
    pub fn new(values: &[u32]) -> Self {
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

    fn coords(&self) -> Vec<(u32, u32)> {
        let mut result = vec![];
        for x in self.x0..self.x1 {
            for y in self.y0..self.y1 {
                result.push((x, y))
            }
        }
        result
    }
}

#[derive(Deserialize)]
struct PatternDefinition {
    tiles: Vec<TilesDefinition>,
}

impl PatternDefinition {
    fn tiles(&self) -> Iter<'_, TilesDefinition> {
        self.tiles.iter()
    }
}

#[derive(Deserialize)]
pub struct TilesDefinition {
    #[serde(alias = "name")]
    sprite: Option<Sprite>,
    #[serde(alias = "pattern")]
    pattern: Option<String>,
    #[serde(alias = "type")]
    kind: Option<Kind>,
    animation: Option<AnimationName>,
    ranges: Vec<Vec<u32>>,
}

impl TilesDefinition {
    pub fn sprite(&self) -> Option<Sprite> {
        self.sprite
    }
    pub fn pattern(&self) -> Option<String> {
        self.pattern.clone()
    }
    pub fn coords(&self) -> Vec<(u32, u32)> {
        self.ranges
            .iter()
            .map(|v| Ranges::new(v))
            .flat_map(|r| r.coords())
            .collect()
    }

    fn create_tile_data(&self, x: u32, y: u32, tile_size: Size) -> TileData {
        let sprite = self.sprite.unwrap();
        let left = (x * tile_size.width) as f64;
        let top = (y * tile_size.height) as f64;
        let rectangle = BBox::new(top, left, tile_size);
        TileData::new(sprite, (x, y), self.kind, self.animation, rectangle)
    }
}

//
#[derive(Copy, Clone, Debug)]
pub struct TileData {
    sprite: Sprite,
    position: (u32, u32),
    tile: Option<Kind>,
    animation: Option<AnimationName>,
    rectangle: BBox,
}

impl TileData {
    pub fn new(
        sprite: Sprite,
        position: (u32, u32),
        tile: Option<Kind>,
        animation: Option<AnimationName>,
        rectangle: BBox,
    ) -> Self {
        Self {
            sprite,
            position,
            tile,
            animation,
            rectangle,
        }
    }

    pub fn replace_sprite(&self, sprite: Sprite) -> TileData {
        TileData::new(
            sprite,
            self.position,
            self.tile,
            self.animation,
            self.rectangle,
        )
    }

    pub fn sprite(&self) -> Sprite {
        self.sprite
    }

    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    pub fn tile(&self) -> Option<Kind> {
        self.tile
    }
    pub fn animation(&self) -> Option<AnimationName> {
        self.animation
    }
    pub fn rectangle(&self) -> BBox {
        self.rectangle
    }

    pub fn offset(&mut self, x: u32, y: u32) {
        let (x0, y0) = self.position;
        self.position = (x0 + x, y0 + y);
        self.rectangle = self
            .rectangle
            .translate((x * TILE_SIZE) as f64, (y * TILE_SIZE) as f64);
    }
}

type XYTileData = (u32, u32, TileData);

#[derive(Deserialize)]
struct LayerDefinition {
    tiles: Vec<TilesDefinition>,
}

#[derive(Deserialize)]
pub struct EntityDefinition {
    name: String,
    pos: Position,
}

impl EntityDefinition {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn position(&self) -> Position {
        self.pos
    }
}

#[derive(Deserialize)]
pub struct LevelDefinition {
    #[serde(alias = "spriteSheet")]
    sprite_sheet: String,
    #[serde(alias = "musicSheet")]
    music_sheet: String,
    #[serde(alias = "tileSize")]
    tile_size: Option<Size>,
    layers: Vec<LayerDefinition>,
    patterns: HashMap<String, PatternDefinition>,
    entities: Vec<EntityDefinition>,
    gravity: Option<f64>,
}

impl LevelDefinition {
    pub async fn load(name: &str) -> Result<LevelDefinition, JsValue> {
        log(&format!("Loading level {}", name));
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

    pub fn music_sheet(&self) -> &str {
        self.music_sheet.as_ref()
    }

    pub fn entities(&self) -> &[EntityDefinition] {
        self.entities.as_slice()
    }

    fn create_pattern_tiles_data(&self, pattern_definition: &PatternDefinition) -> Vec<XYTileData> {
        let mut result = vec![];
        for tiles in pattern_definition.tiles() {
            for (x, y) in tiles.coords() {
                for &(offset_x, offset_y, mut data) in self.get_tiles_data(tiles).iter() {
                    data.offset(x, y);
                    result.push((x + offset_x, y + offset_y, data));
                }
            }
        }
        result
    }

    fn get_tiles_data(&self, tiles: &TilesDefinition) -> Vec<XYTileData> {
        if tiles.sprite().is_some() {
            let tile_size = self
                .tile_size
                .unwrap_or_else(|| Size::new(TILE_SIZE, TILE_SIZE));
            let data = tiles.create_tile_data(0, 0, tile_size);
            vec![(0, 0, data)]
        } else if tiles.pattern().is_some() {
            let pattern_name = tiles.pattern().unwrap();
            let pattern = self
                .patterns
                .get(&pattern_name)
                .unwrap_or_else(|| panic!("Missing pattern: {}", pattern_name));
            self.create_pattern_tiles_data(pattern)
        } else {
            panic!("Expected a tile or a pattern!")
        }
    }
    fn compute_tiles_data(&self) -> Vec<Vec<XYTileData>> {
        let mut result = vec![];
        for layer in self.layers.iter() {
            let mut layer_vec = vec![];
            for tiles in layer.tiles.iter() {
                for (x, y) in tiles.coords() {
                    for &(offset_x, offset_y, mut data) in self.get_tiles_data(tiles).iter() {
                        data.offset(x, y);
                        layer_vec.push((x + offset_x, y + offset_y, data))
                    }
                }
            }
            result.push(layer_vec);
        }
        result
    }

    fn compute_size(&self, data: &[Vec<XYTileData>]) -> Size {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        for v in data.iter() {
            for (x, y, _data) in v.iter() {
                width = width.max(*x);
                height = height.max(*y);
            }
        }
        Size::new(width + 1, height + 1)
    }

    fn create_background_matrix(&self, size: Size, data: Vec<XYTileData>) -> Matrix<TileData> {
        let mut matrix = Matrix::new(size);
        for (x, y, data) in data {
            matrix.set(x as usize, y as usize, data);
        }
        matrix
    }

    pub async fn build(
        &self,
    ) -> Result<(Vec<Rc<RefCell<Matrix<TileData>>>>, SpriteSheet, Option<f64>), JsValue> {
        let all_tile_data = self.compute_tiles_data();

        let size = self.compute_size(&all_tile_data);
        let mut backgrounds_matrix = vec![];
        for v in all_tile_data {
            let background_matrix = self.create_background_matrix(size, v.to_vec());
            backgrounds_matrix.push(Rc::new(RefCell::new(background_matrix)));
        }

        let sprite_sheet = SpriteSheet::load(self.sprite_sheet.as_str()).await?;

        Ok((backgrounds_matrix, sprite_sheet, self.gravity))
    }
}
