use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlImageElement;

pub mod config;
pub mod font;
pub mod levels;
pub mod sprites;

pub const TILE_SIZE: u32 = 16;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = loader)]
    fn loadImage(path: &str) -> Promise;
}

async fn load_image(path: &str) -> Result<HtmlImageElement, JsValue> {
    let promise = loadImage(path);
    let result = JsFuture::from(promise).await?;
    let image = result.dyn_into::<HtmlImageElement>().unwrap();

    Ok(image)
}
