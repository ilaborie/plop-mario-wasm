use std::collections::HashMap;
use std::rc::Rc;

use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::assets::animations::{Animation, AnimationDefinition, AnimationName};
use crate::assets::{load_image, load_json, TILE_SIZE};
use crate::physics::{Direction, Size};
use crate::utils::{create_image_buffer, log};

#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprite {
    // Mario
    #[serde(alias = "idle")]
    Idle,
    #[serde(alias = "run-1")]
    Run1,
    #[serde(alias = "run-2")]
    Run2,
    #[serde(alias = "run-3")]
    Run3,
    #[serde(alias = "break")]
    Break,
    #[serde(alias = "dead")]
    Dead,
    #[serde(alias = "jump")]
    Jump,
    // Goomba, Koopa
    #[serde(alias = "walk-1")]
    Walk1,
    #[serde(alias = "walk-2")]
    Walk2,
    // Goomba
    #[serde(alias = "flat")]
    Flat,
    // Koopa
    #[serde(alias = "hiding")]
    Hiding,
    #[serde(alias = "hiding-with-legs")]
    HidingWithLegs,
    // Level
    #[serde(alias = "ground")]
    Ground,
    #[serde(alias = "sky")]
    Sky,
    #[serde(alias = "chance")]
    Chance,
    #[serde(alias = "coin")]
    Coin,
    #[serde(alias = "bricks")]
    Brick,
    #[serde(alias = "bricks-broken")]
    BrickBroken,
    #[serde(alias = "chocolate")]
    Chocolate,
    // Pipes
    #[serde(alias = "pipe-insert-vert-left")]
    PipeCapLeft,
    #[serde(alias = "pipe-insert-vert-right")]
    PipeCapRight,
    #[serde(alias = "pipe-vert-left")]
    PipeLeft,
    #[serde(alias = "pipe-vert-right")]
    PipeRight,
    // Cloud
    #[serde(alias = "cloud-1-1")]
    Cloud11,
    #[serde(alias = "cloud-1-2")]
    Cloud12,
    #[serde(alias = "cloud-1-3")]
    Cloud13,
    #[serde(alias = "cloud-2-1")]
    Cloud21,
    #[serde(alias = "cloud-2-2")]
    Cloud22,
    #[serde(alias = "cloud-2-3")]
    Cloud23,
    // Chance
    #[serde(alias = "chance-1")]
    Chance1,
    #[serde(alias = "chance-2")]
    Chance2,
    #[serde(alias = "chance-3")]
    Chance3,
    // Coin
    #[serde(alias = "coin-1")]
    Coin1,
    #[serde(alias = "coin-2")]
    Coin2,
    #[serde(alias = "coin-3")]
    Coin3,
    // Cannon
    #[serde(alias = "cannon-1")]
    Cannon1,
    #[serde(alias = "cannon-2")]
    Cannon2,
    #[serde(alias = "cannon-3")]
    Cannon3,
    #[serde(alias = "bullet")]
    Bullet,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32, size: Size) -> Self {
        let width = size.width;
        let height = size.height;

        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn size(&self) -> Size {
        let width = self.width;
        let height = self.height;

        Size::new(width, height)
    }
}

#[derive(Deserialize)]
struct FrameDefinition {
    name: Sprite,
    rect: Rectangle,
}

#[derive(Deserialize)]
struct TileDefinition {
    name: Sprite,
    index: (u32, u32),
}

#[derive(Deserialize)]
struct SpriteSheetDefinition {
    #[serde(alias = "imageURL")]
    image_url: String,
    #[serde(alias = "tileW")]
    tile_width: Option<u32>,
    #[serde(alias = "tileH")]
    tile_height: Option<u32>,
    #[serde(default)]
    tiles: Vec<TileDefinition>,
    #[serde(default)]
    frames: Vec<FrameDefinition>,
    #[serde(default)]
    animations: Vec<AnimationDefinition>,
}

impl SpriteSheetDefinition {
    pub async fn load(name: &str) -> Result<SpriteSheetDefinition, JsValue> {
        log(&format!("Loading sprite sheet '{}'", name));

        let url = format!("assets/sprites/{}.json", name);
        let result = load_json(url.as_str())
            .await?
            .into_serde::<SpriteSheetDefinition>()
            .expect("Error during sprites loading");

        Ok(result)
    }
}

// SpriteSheet
pub struct SpriteSheet {
    name: String,
    image: Rc<HtmlImageElement>,
    tile_size: Size,
    sprites: HashMap<Sprite, HtmlCanvasElement>,
    sprites_size: HashMap<Sprite, Size>,
    animations: HashMap<AnimationName, Animation>,
}

impl SpriteSheet {
    pub async fn load(name: &str) -> Result<SpriteSheet, JsValue> {
        let definition = SpriteSheetDefinition::load(name).await?;

        let name = String::from(name);
        let tile_width = definition.tile_width.unwrap_or(TILE_SIZE);
        let tile_height = definition.tile_height.unwrap_or(TILE_SIZE);
        let tile_size = Size::new(tile_width, tile_height);

        let image = load_image(&definition.image_url).await?;

        let mut result = SpriteSheet::new(name, image.clone(), tile_size);

        // Tiles
        for tile_def in definition.tiles.iter() {
            let (x, y) = tile_def.index;
            let rect = Rectangle::new(x * tile_width, y * tile_height, tile_size);
            result.define(tile_def.name, &rect);
        }

        // Animations
        for animation_def in definition.animations.iter() {
            let mut animation =
                Animation::build(animation_def.name(), animation_def, image.clone());
            // Define tiles
            for tile_def in definition.tiles.iter() {
                if animation_def.frames().contains(&tile_def.name) {
                    let (x, y) = tile_def.index;
                    let x = x * tile_width;
                    let y = y * tile_height;
                    let width = tile_width;
                    let height = tile_height;
                    let rect = Rectangle {
                        x,
                        y,
                        width,
                        height,
                    };
                    animation.define(tile_def.name, Direction::Right, &rect);
                }
            }
            // Define all frames
            for &d in [Direction::Left, Direction::Stop, Direction::Right].iter() {
                for frame_def in definition.frames.iter() {
                    animation.define(frame_def.name, d, &frame_def.rect);
                    result.define(frame_def.name, &frame_def.rect);
                }
            }
            result.animations.insert(animation_def.name(), animation);
        }

        Ok(result)
    }

    fn new(name: String, image: Rc<HtmlImageElement>, tile_size: Size) -> Self {
        let sprites = HashMap::default();
        let sprites_size = HashMap::default();
        let animations = HashMap::new();
        Self {
            name,
            image,
            tile_size,
            sprites,
            sprites_size,
            animations,
        }
    }

    fn define(&mut self, frame: Sprite, rect: &Rectangle) {
        let buffer = create_image_buffer(self.image.clone(), rect, false);
        self.sprites.insert(frame, buffer);
        self.sprites_size.insert(frame, rect.size());
    }

    pub fn draw_tile_animation(
        &self,
        context: &CanvasRenderingContext2d,
        animation: AnimationName,
        x: f64,
        y: f64,
        distance: f64,
        direction: Direction,
    ) {
        let anim = self
            .animations
            .get(&animation)
            .unwrap_or_else(|| panic!("[{}] Animation {:?} not found!", self.name, animation));
        let frame = anim.frame(distance);
        anim.draw_frame(context, x, y, frame, direction);
    }

    pub fn draw_tile_animation_fixed(
        &self,
        context: &CanvasRenderingContext2d,
        animation: AnimationName,
        frame: Sprite,
        x: f64,
        y: f64,
        direction: Direction,
    ) {
        let anim = self
            .animations
            .get(&animation)
            .unwrap_or_else(|| panic!("[{}] Animation {:?} not found!", self.name, animation));
        anim.draw_frame(context, x, y, frame, direction);
    }

    fn draw_image(&self, context: &CanvasRenderingContext2d, sprite: Sprite, x: f64, y: f64) {
        let buffer = self
            .sprites
            .get(&sprite)
            .unwrap_or_else(|| panic!("[{}] Sprite {:?} not found!", self.name, sprite));
        context
            .draw_image_with_html_canvas_element(buffer, x.floor(), y.floor())
            .unwrap();
    }

    pub fn size(&self, sprite: Sprite) -> Size {
        self.sprites_size
            .get(&sprite)
            .copied()
            .unwrap_or_else(|| panic!("[{}] Sprite size {:?} not found!", self.name, sprite))
    }

    pub fn tile_size(&self) -> Size {
        self.tile_size
    }

    pub fn draw_tile(&self, context: &CanvasRenderingContext2d, sprite: Sprite, x: f64, y: f64) {
        let Size { width, height } = self.tile_size;
        let x = x * width as f64;
        let y = y * height as f64;
        self.draw_image(context, sprite, x, y);
    }
}
