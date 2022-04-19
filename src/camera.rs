use core::cell::RefCell;
use std::rc::Rc;

use crate::physics::{Position, Size};

pub struct Camera {
    position: Rc<RefCell<Position>>,
    size: Size,
}

impl Camera {
    pub fn new(size: Size) -> Self {
        let position = Rc::new(RefCell::new(Position::default()));
        Camera { position, size }
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn position(&self) -> (f64, f64) {
        (self.position.borrow().x(), self.position.borrow().y())
    }

    pub fn set_x(&mut self, x: f64) {
        self.position.borrow_mut().set_x(x);
    }

    // pub fn set_y(&mut self, y: f64) {
    //     self.position.borrow_mut().set_y(y);
    // }
}
