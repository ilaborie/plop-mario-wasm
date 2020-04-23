use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use crate::assets::sprites::{SpriteSheet, Sprite};

pub mod sprites;
pub mod levels;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = loader)]
    fn loadImage(path: &str) -> Promise;
}

async fn load_image(path: &str) -> Result<HtmlImageElement, JsValue> {
    let promise = loadImage(path);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    let image = result
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    Ok(image)
}

pub async fn load_background_sprites() -> Result<SpriteSheet, JsValue> {
    let img = load_image("assets/tiles.png").await?;
    let mut sprites = SpriteSheet::new(img, 16, 16);
    sprites.define_tile(Sprite::Ground, 0, 0);
    sprites.define_tile(Sprite::Sky, 3, 23);

    Ok(sprites)
}

pub async fn load_player_sprites() -> Result<SpriteSheet, JsValue> {
    let img = load_image("assets/characters.gif").await?;
    let mut sprites = SpriteSheet::new(img, 16, 16);
    sprites.define(Sprite::MarioIdle, 276, 44, 16, 16);

    Ok(sprites)
}

