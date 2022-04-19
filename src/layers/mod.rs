use std::cell::RefCell;
use std::rc::Rc;

use web_sys::CanvasRenderingContext2d;

use crate::camera::Camera;

pub mod backgrounds;
pub mod collision;
pub mod colors;
pub mod dashboard;
pub mod entity;
pub mod player_progress;

pub trait Drawable {
    fn draw(&mut self, context: Rc<CanvasRenderingContext2d>, camera: &Camera);
    //fn draw(&mut self, game_context: &GameContext, level: &Level);
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
    fn draw(&mut self, context: Rc<CanvasRenderingContext2d>, camera: &Camera) {
        for layer in self.layers.iter() {
            layer.borrow_mut().draw(context.clone(), camera);
        }
    }
}
