use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d};
use std::collections::HashMap;
use crate::levels::{Level, Background};

#[wasm_bindgen]
#[repr(u32)]
#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprite {
    Ground = 0,
    Sky = 1,
}

#[wasm_bindgen]
pub struct SpriteSheet {
    image: HtmlImageElement,
    width: u32,
    height: u32,
    sprites: HashMap<Sprite, HtmlCanvasElement>,
}

#[wasm_bindgen]
impl SpriteSheet {
    pub fn new(image: HtmlImageElement, width: u32, height: u32) -> Self {
        let sprites = HashMap::new();
        Self { image, width, height, sprites }
    }

    pub fn define(&mut self, sprite: Sprite, x: u32, y: u32) {
        let document = web_sys::window().unwrap().document().unwrap();
        let buffer = document.create_element("canvas").unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        buffer.set_width(self.width);
        buffer.set_height(self.height);

        let context = buffer
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self.image,
            (x * self.width) as f64, (y * self.width) as f64,
            self.width as f64, self.width as f64,
            0.0, 0.0,
            self.width as f64, self.width as f64).unwrap();

        self.sprites.insert(sprite, buffer);
    }

    fn draw_image(&self, sprite: Sprite, context: &CanvasRenderingContext2d, x: u32, y: u32) {
        let buffer = self.sprites.get(&sprite).unwrap();
        context.draw_image_with_html_canvas_element(&buffer, x as f64, y as f64).unwrap();
    }

    fn draw_tile(&self, sprite: Sprite, context: &CanvasRenderingContext2d, x: u32, y: u32) {
        self.draw_image(sprite, context, x * self.width, y * self.height);
    }

    fn draw_background(&self, context: &CanvasRenderingContext2d, background: &Background) {
        for x in background.ranges().x() {
            for y in background.ranges().y() {
                self.draw_tile(background.tile(), context, x, y);
            }
        }
    }

    pub fn draw_level(&self, context: &CanvasRenderingContext2d, level: &JsValue) {
        let level = level.into_serde::<Level>().unwrap();
        // backgrounds
        level.backgrounds().for_each(|bg| self.draw_background(context, bg));
    }
}
