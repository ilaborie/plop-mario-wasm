use crate::layers::Layer;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct Compositor {
    layers: Vec<Rc<Layer>>,
}

impl Compositor {
    pub fn new() -> Self {
        let layers = vec![];
        Self { layers }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        for layer in self.layers.iter() {
            layer(context);
        }
    }

    pub fn add_layer(&mut self, layer: Rc<Layer>) {
        self.layers.push(layer);
    }
}
