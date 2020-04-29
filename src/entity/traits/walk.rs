use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::bounding_box::BoundingBox;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Walk {
    speed: f64,
    enable: bool,
}

impl Walk {
    pub fn new(speed: f64) -> Self {
        let enable = true;
        Self { speed, enable }
    }
    pub fn speed(&self) -> f64 {
        self.speed
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
    pub fn enable(&mut self) {
        self.enable = true;
    }
    pub fn disable(&mut self) {
        self.enable = false;
    }
}

impl EntityTrait for Walk {
    fn update(&mut self, entity: Rc<RefCell<Entity>>, _dt: f64) {
        // Move X
        if self.enable {
            entity.borrow_mut().dx = self.speed;
        }
    }

    fn obstruct(&mut self, entity: Rc<RefCell<Entity>>, side: ObstructionSide, rect: BoundingBox) {
        match side {
            ObstructionSide::Right => {
                let x = rect.left() - entity.borrow().size.width as f64;
                self.speed *= -1.;
                entity.borrow_mut().x = x;
            }
            ObstructionSide::Left => {
                let x = rect.right();
                self.speed *= -1.;
                entity.borrow_mut().x = x;
            }
            ObstructionSide::Top => {
                let y = rect.bottom();
                entity.borrow_mut().set_y(y, 0.);
            }
            ObstructionSide::Bottom => {
                let y = rect.top() - entity.borrow().size.height as f64;
                entity.borrow_mut().set_y(y, 0.);
            }
        }
    }
}
