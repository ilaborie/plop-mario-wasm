use web_sys::CanvasRenderingContext2d;
use std::rc::Rc;

pub mod backgrounds;
pub mod player;

pub type Layer = dyn Fn(&CanvasRenderingContext2d) -> ();

#[derive(Default)]
pub struct Compositor {
    layers: Vec<Rc<Layer>>,
}

impl Compositor {

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        for layer in self.layers.iter() {
            layer(context);
        }
    }

    pub fn add_layer(&mut self, layer: Rc<Layer>) {
        self.layers.push(layer);
    }
}
