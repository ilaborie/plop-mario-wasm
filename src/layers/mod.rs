use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub mod backgrounds;
pub mod collision;
pub mod level;

pub type Layer = dyn Fn(&CanvasRenderingContext2d) -> ();

pub trait Drawable {
    fn draw(&self, context: &CanvasRenderingContext2d);
}

#[derive(Default)]
pub struct Compositor {
    layers: Vec<Rc<Layer>>,
}

impl Compositor {
    pub fn add_layer(&mut self, layer: Rc<Layer>) {
        self.layers.push(layer);
    }
}

impl Drawable for Compositor {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        for layer in self.layers.iter() {
            layer(context);
        }
    }
}
