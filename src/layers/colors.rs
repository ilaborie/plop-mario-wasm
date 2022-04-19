use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct ColorsLayer {
    color: String,
}

impl ColorsLayer {
    pub fn new(color: &str) -> Self {
        let color = String::from(color);
        Self { color }
    }

    pub fn draw(&mut self, context: Rc<CanvasRenderingContext2d>) {
        let canvas = context.canvas().unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        context.set_fill_style(&JsValue::from(self.color.as_str()));
        context.set_stroke_style(&JsValue::from(self.color.as_str()));
        context.fill_rect(0., 0., width, height);
    }
}
