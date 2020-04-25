use crate::assets::{load_image, TILE_SIZE};
use crate::utils::{create_buffer, window};
use core::fmt;
use fmt::Formatter;
use std::collections::HashMap;
use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Request, Response};

#[derive(Serialize, Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprite {
    // Tiles
    #[serde(alias = "ground")]
    Ground,
    #[serde(alias = "sky")]
    Sky,
    #[serde(alias = "chance")]
    Chance,
    #[serde(alias = "bricks")]
    Bricks,
    #[serde(alias = "chocolate")]
    Chocolate,
    // Mario
    MarioIdle,
}

impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Sprite::Ground => "🟫",
            Sprite::Sky => "🟦",
            Sprite::Chance => "🍀",
            Sprite::Bricks => "🔳",
            Sprite::Chocolate => "🟤",
            Sprite::MarioIdle => "🦄",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize)]
struct SpriteDefinition {
    tile: Sprite,
    x: u32,
    y: u32,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct SpriteSheetDefinition {
    image: String,
    width: Option<u32>,
    height: Option<u32>,
    sprites: Vec<SpriteDefinition>,
}

impl SpriteSheetDefinition {
    pub async fn load(name: &str) -> Result<SpriteSheetDefinition, JsValue> {
        // log(&format!("Loading sprite sheet {}", name).to_string());
        let url = format!("/assets/sprites/{}.json", name);
        let request = Request::new_with_str(&url)?;

        let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;

        let level = json
            .into_serde::<SpriteSheetDefinition>()
            .expect("Error during level loading");

        Ok(level)
    }
}

pub struct SpriteSheet {
    image: HtmlImageElement,
    width: u32,
    height: u32,
    sprites: HashMap<Sprite, HtmlCanvasElement>,
}

impl SpriteSheet {
    pub async fn load(name: &str) -> Result<SpriteSheet, JsValue> {
        let definition = SpriteSheetDefinition::load(name).await?;

        let img = load_image(&definition.image).await?;
        let width = definition.width.unwrap_or(TILE_SIZE);
        let height = definition.height.unwrap_or(TILE_SIZE);

        let mut result = SpriteSheet::new(img, width, height);
        for sprite_def in definition.sprites.iter() {
            let w = sprite_def.width.unwrap_or(width);
            let h = sprite_def.width.unwrap_or(height);
            result.define(sprite_def.tile, sprite_def.x, sprite_def.y, w, h);
        }

        Ok(result)
    }

    fn new(image: HtmlImageElement, width: u32, height: u32) -> Self {
        let sprites = HashMap::new();
        Self {
            image,
            width,
            height,
            sprites,
        }
    }

    fn define(&mut self, sprite: Sprite, x: u32, y: u32, width: u32, height: u32) {
        let buffer = create_buffer(width, height, |context| {
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image,
                    x as f64,
                    y as f64,
                    width as f64,
                    height as f64,
                    0.0,
                    0.0,
                    width as f64,
                    height as f64,
                )
                .unwrap()
        });

        self.sprites.insert(sprite, buffer);
    }

    pub fn draw_image(&self, context: &CanvasRenderingContext2d, sprite: &Sprite, x: f64, y: f64) {
        let buffer = self.sprites.get(&sprite).unwrap();
        context
            .draw_image_with_html_canvas_element(&buffer, x as f64, y as f64)
            .unwrap();
    }

    pub fn draw_tile(&self, context: &CanvasRenderingContext2d, sprite: &Sprite, x: f64, y: f64) {
        self.draw_image(
            context,
            sprite,
            x * self.width as f64,
            y * self.height as f64,
        );
    }
}
