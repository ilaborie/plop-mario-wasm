use crate::assets::animations::{Animation, AnimationDefinition, AnimationName};
use crate::assets::{load_image, TILE_SIZE};
use crate::entity::animation::AnimationEntity;
use crate::physics::motion::Direction;
use crate::utils::{create_image_buffer, log, window};
use core::fmt;
use fmt::Formatter;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Request, Response};

#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprite {
    // Tiles
    #[serde(alias = "ground")]
    Ground,
    #[serde(alias = "sky")]
    Sky,
    #[serde(alias = "chance")]
    Chance,
    #[serde(alias = "coin")]
    Coin,
    #[serde(alias = "bricks")]
    Bricks,
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
    // Cannon
    #[serde(alias = "canon-1")]
    Cannon1,
    #[serde(alias = "canon-2")]
    Cannon2,
    #[serde(alias = "canon-3")]
    Cannon3,
}

impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Sprite::Ground => "🟫",
            Sprite::Sky => "🟦",
            Sprite::Chance => "🍀",
            Sprite::Bricks => "🔳",
            Sprite::Chocolate => "🟤",
            Sprite::PipeCapLeft | Sprite::PipeCapRight => "🟢",
            Sprite::PipeLeft | Sprite::PipeRight => "🟩",
            Sprite::Cannon1 | Sprite::Cannon2 | Sprite::Cannon3 => "💣",
            Sprite::Cloud11
            | Sprite::Cloud12
            | Sprite::Cloud13
            | Sprite::Cloud21
            | Sprite::Cloud22
            | Sprite::Cloud23 => "☁️",
            Sprite::Coin => "💰",
        };
        write!(f, "{}", s)
    }
}

#[derive(Deserialize)]
struct SpriteDefinition {
    tile: Sprite,
    x: u32,
    y: u32,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Deserialize)]
struct SpriteSheetDefinition {
    image: String,
    width: Option<u32>,
    height: Option<u32>,
    sprites: Vec<SpriteDefinition>,
    animations: Vec<AnimationDefinition>,
}

impl SpriteSheetDefinition {
    pub async fn load(name: &str) -> Result<SpriteSheetDefinition, JsValue> {
        log(&format!("Loading json {}", name).to_string());
        let url = format!("/assets/sprites/{}.json", name);
        let request = Request::new_with_str(&url)?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let result = json
            .into_serde::<SpriteSheetDefinition>()
            .expect("Error during level loading");

        Ok(result)
    }
}

pub struct SpriteSheet {
    image: Rc<RefCell<HtmlImageElement>>,
    width: u32,
    height: u32,
    sprites: HashMap<Sprite, HtmlCanvasElement>,
    animations: HashMap<AnimationName, Animation>,
}

impl SpriteSheet {
    pub async fn load(name: &str) -> Result<SpriteSheet, JsValue> {
        let definition = SpriteSheetDefinition::load(name).await?;

        let image = Rc::new(RefCell::new(load_image(&definition.image).await?));
        let width = definition.width.unwrap_or(TILE_SIZE);
        let height = definition.height.unwrap_or(TILE_SIZE);

        let mut result = SpriteSheet::new(image.clone(), width, height);

        // Sprites
        for sprite_def in definition.sprites.iter() {
            let w = sprite_def.width.unwrap_or(width);
            let h = sprite_def.height.unwrap_or(height);
            result.define(sprite_def.tile, sprite_def.x, sprite_def.y, w, h);
        }

        // Animations
        for animation_def in definition.animations.iter() {
            let animation = Animation::build(animation_def, image.clone());
            result.animations.insert(animation_def.name, animation);
        }

        Ok(result)
    }

    fn new(image: Rc<RefCell<HtmlImageElement>>, width: u32, height: u32) -> Self {
        let sprites = HashMap::new();
        let animations = HashMap::new();
        Self {
            image,
            width,
            height,
            sprites,
            animations,
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }
    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    fn define(&mut self, sprite: Sprite, x: u32, y: u32, width: u32, height: u32) {
        let buffer =
            create_image_buffer(self.image.clone(), x as f64, y as f64, width, height, false);
        self.sprites.insert(sprite, buffer);
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
        let anim = self.animations.get(&animation).unwrap();
        let frame = anim.frame(distance);
        anim.draw_frame(context, x, y, frame, direction);
    }
    pub fn draw_entity_animation(
        &self,
        context: &CanvasRenderingContext2d,
        animation: AnimationName,
        x: f64,
        y: f64,
        entity: Rc<RefCell<AnimationEntity>>,
    ) {
        let anim = self.animations.get(&animation).unwrap();
        let frame = anim.entity_frame(entity.clone());
        anim.draw_frame(context, x, y, frame, entity.borrow().direction());
    }

    fn draw_image(&self, context: &CanvasRenderingContext2d, sprite: &Sprite, x: f64, y: f64) {
        let buffer = self.sprites.get(&sprite).unwrap();
        context
            .draw_image_with_html_canvas_element(&buffer, x as f64, y as f64)
            .unwrap();
    }

    pub(crate) fn draw_tile(
        &self,
        context: &CanvasRenderingContext2d,
        sprite: &Sprite,
        x: f64,
        y: f64,
    ) {
        self.draw_image(
            context,
            sprite,
            x * self.width as f64,
            y * self.height as f64,
        );
    }
}
