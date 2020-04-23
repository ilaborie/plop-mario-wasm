use crate::entity::{Velocity, Position, Jumping};
use std::rc::Rc;
use std::cell::RefCell;

pub enum EntityTrait {
    Velocity { position: Rc<RefCell<Position>>, velocity: Rc<RefCell<Velocity>> },
    Gravity { gravity: f64, velocity: Rc<RefCell<Velocity>> },
    Jump { velocity: Rc<RefCell<Velocity>>, jumping: Rc<RefCell<Jumping>> },
}

impl EntityTrait {
    pub fn new_velocity(position: Rc<RefCell<Position>>, velocity: Rc<RefCell<Velocity>>) -> EntityTrait {
        EntityTrait::Velocity { position, velocity }
    }

    pub fn new_gravity(velocity: Rc<RefCell<Velocity>>, gravity: f64) -> EntityTrait {
        EntityTrait::Gravity { velocity, gravity }
    }

    pub fn new_jump(velocity: Rc<RefCell<Velocity>>, jumping: Rc<RefCell<Jumping>>) -> EntityTrait {
        EntityTrait::Jump { velocity, jumping }
    }

    pub fn update(&mut self, dt: f64) {
        match self {
            EntityTrait::Velocity { position, velocity } => {
                position.borrow_mut().x += velocity.borrow().dx * dt;
                position.borrow_mut().y += velocity.borrow().dy * dt;
            }
            EntityTrait::Gravity { gravity, velocity } => {
                velocity.borrow_mut().dy += *gravity * dt;
            }
            EntityTrait::Jump { velocity, jumping } => {
                if jumping.borrow().engage_time > 0.0 {
                    velocity.borrow_mut().dy += -jumping.borrow().velocity;
                    jumping.borrow_mut().engage_time -= dt;
                }
            }
        }
    }
}
