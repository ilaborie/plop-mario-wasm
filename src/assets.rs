use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d};
use serde::Deserialize;
use std::collections::HashMap;
use crate::utils::create_buffer;

#[wasm_bindgen]
#[repr(u32)]
#[derive(Deserialize, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Sprite {
    // Tiles
    Ground = 0,
    Sky = 1,
    // Mario
    MarioIdle = 1000,
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

    pub fn define_tile(&mut self, sprite: Sprite, x: u32, y: u32) {
        self.define(sprite, x * self.width, y * self.height, self.width, self.height);
    }
    pub fn define(&mut self, sprite: Sprite, x: u32, y: u32, width: u32, height: u32) {
        let buffer = create_buffer(width, height, |context|
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.image,
                x as f64, y as f64,
                width as f64, height as f64,
                0.0, 0.0,
                width as f64, height as f64).unwrap(),
        );

        self.sprites.insert(sprite, buffer);
    }

    pub fn draw_image(&self, context: &CanvasRenderingContext2d, sprite: Sprite, x: u32, y: u32) {
        let buffer = self.sprites.get(&sprite).unwrap();
        context.draw_image_with_html_canvas_element(&buffer, x as f64, y as f64).unwrap();
    }

    pub fn draw_tile(&self, context: &CanvasRenderingContext2d, sprite: Sprite, x: u32, y: u32) {
        self.draw_image(context, sprite, x * self.width, y * self.height);
    }

}
