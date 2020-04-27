use crate::camera::Camera;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;
pub mod backgrounds;
pub mod camera;
pub mod collision;
pub mod entity;

pub trait Drawable {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>);
}

#[derive(Default)]
pub struct Compositor {
    layers: Vec<Rc<RefCell<dyn Drawable>>>,
}

impl Compositor {
    pub fn add_layer(&mut self, layer: Rc<RefCell<dyn Drawable>>) {
        self.layers.push(layer);
    }
}

impl Drawable for Compositor {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        for layer in self.layers.iter() {
            layer.borrow_mut().draw(context, camera.clone());
        }
    }
}
