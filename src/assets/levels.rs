use crate::assets::animations::AnimationName;
use crate::assets::patterns::{load_patterns, PatternDefinition};
use crate::assets::sprites::{Sprite, SpriteSheet};
use crate::assets::tiles::TilesDefinition;
use crate::assets::{load_json, TILE_SIZE};
use crate::physics::bounding_box::BBox;
use crate::physics::matrix::Matrix;
use crate::physics::{Position, Size};
use crate::utils::log;
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileType {
    #[serde(alias = "ground")]
    Ground,
    #[serde(alias = "brick")]
    Brick,
    #[serde(alias = "brick-broken")]
    BrickBroken,
    #[serde(alias = "coin")]
    Coin,
}

//
#[derive(Copy, Clone, Debug)]
pub struct TileData {
    sprite: Sprite,
    position: (u32, u32),
    tile: Option<TileType>,
    animation: Option<AnimationName>,
    rectangle: BBox,
}

impl TileData {
    pub fn new(
        sprite: Sprite,
        position: (u32, u32),
        tile: Option<TileType>,
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

    pub fn tile(&self) -> Option<TileType> {
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

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Copy, Clone, Debug)]
pub enum TriggerKind {
    #[serde(alias = "goto")]
    Goto,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TriggerDefinition {
    name: String,
    #[serde(alias = "type")]
    kind: TriggerKind,
    pos: (u32, u32),
}

impl TriggerDefinition {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn kind(&self) -> TriggerKind {
        self.kind
    }
    pub fn position(&self) -> (u32, u32) {
        self.pos
    }
}

pub struct LevelSpec {
    tiles: Vec<Rc<RefCell<Matrix<TileData>>>>,
    music: String,
    bg_sprites: Rc<SpriteSheet>,
    gravity: Option<f64>,
    entities: Vec<EntityDefinition>,
    triggers: Vec<TriggerDefinition>,
}

impl LevelSpec {
    pub fn music(&self) -> &str {
        self.music.as_str()
    }
    pub fn tiles(&self) -> Vec<Rc<RefCell<Matrix<TileData>>>> {
        self.tiles.clone()
    }
    pub fn bg_sprites(&self) -> Rc<SpriteSheet> {
        self.bg_sprites.clone()
    }
    pub fn gravity(&self) -> Option<f64> {
        self.gravity
    }
    pub fn entities(&self) -> Vec<EntityDefinition> {
        self.entities.clone()
    }
    pub fn triggers(&self) -> Vec<TriggerDefinition> {
        self.triggers.clone()
    }
}

#[derive(Deserialize)]
pub struct LevelDefinition {
    gravity: Option<f64>,
    #[serde(alias = "spriteSheet")]
    sprite_sheet: String,
    #[serde(alias = "musicSheet")]
    music_sheet: String,
    #[serde(alias = "patternSheet")]
    pattern_sheet: String,
    tile_size: Option<Size>,
    layers: Vec<LayerDefinition>,
    entities: Vec<EntityDefinition>,
    triggers: Vec<TriggerDefinition>,
}

impl LevelDefinition {
    pub async fn load(name: &str) -> Result<LevelDefinition, JsValue> {
        log(&format!("Loading level {}", name));
        let url = format!("assets/levels/{}.json", name);
        let level = load_json(url.as_str())
            .await?
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

    fn create_pattern_tiles_data(
        &self,
        pattern_definition: &PatternDefinition,
        patterns: &HashMap<String, PatternDefinition>,
    ) -> Vec<XYTileData> {
        let mut result = vec![];
        for tiles in pattern_definition.tiles() {
            for (x, y) in tiles.coords() {
                for &(offset_x, offset_y, mut data) in self.get_tiles_data(tiles, &patterns).iter()
                {
                    data.offset(x, y);
                    result.push((x + offset_x, y + offset_y, data));
                }
            }
        }
        result
    }

    fn get_tiles_data(
        &self,
        tiles: &TilesDefinition,
        patterns: &HashMap<String, PatternDefinition>,
    ) -> Vec<XYTileData> {
        if tiles.sprite().is_some() {
            let tile_size = self
                .tile_size
                .unwrap_or_else(|| Size::new(TILE_SIZE, TILE_SIZE));
            let data = tiles.create_tile_data(0, 0, tile_size);
            vec![(0, 0, data)]
        } else if tiles.pattern().is_some() {
            let pattern_name = tiles.pattern().unwrap();
            let pattern = patterns
                .get(&pattern_name)
                .unwrap_or_else(|| panic!("Missing pattern: {}", pattern_name));
            self.create_pattern_tiles_data(pattern, patterns)
        } else {
            panic!("Expected a tile or a pattern!")
        }
    }
    fn compute_tiles_data(
        &self,
        patterns: &HashMap<String, PatternDefinition>,
    ) -> Vec<Vec<XYTileData>> {
        let mut result = vec![];
        for layer in self.layers.iter() {
            let mut layer_vec = vec![];
            for tiles in layer.tiles.iter() {
                for (x, y) in tiles.coords() {
                    for &(offset_x, offset_y, mut data) in
                        self.get_tiles_data(tiles, patterns).iter()
                    {
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

    pub async fn build(&self) -> Result<LevelSpec, JsValue> {
        let patterns = load_patterns(self.pattern_sheet.as_str()).await?;
        let all_tile_data = self.compute_tiles_data(&patterns);

        let size = self.compute_size(&all_tile_data);
        let mut tiles = vec![];
        for v in all_tile_data {
            let background_matrix = self.create_background_matrix(size, v.to_vec());
            tiles.push(Rc::new(RefCell::new(background_matrix)));
        }

        let music = self.music_sheet.clone();
        let entities = self.entities.clone();
        let sprite_sheet = SpriteSheet::load(self.sprite_sheet.as_str()).await?;
        let bg_sprites = Rc::new(sprite_sheet);
        let gravity = self.gravity;
        let triggers = self.triggers.clone();

        let result = LevelSpec {
            music,
            tiles,
            bg_sprites,
            gravity,
            triggers,
            entities,
        };
        Ok(result)
    }
}
