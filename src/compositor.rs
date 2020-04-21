use web_sys::CanvasRenderingContext2d;
use std::rc::Rc;
use crate::layers::Layer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub(crate) fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub(crate) fn x(&self) -> u32 {
        self.x
    }

    pub(crate) fn y(&self) -> u32 {
        self.y
    }

    pub(crate) fn update(&mut self, incr_x: u32, incr_y: u32) {
        self.x += incr_x;
        self.y += incr_y;
    }
}


// Compositor
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