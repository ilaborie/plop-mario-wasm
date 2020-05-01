use crate::entity::traits::solid::Solid;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

pub struct Killable {
    solid: Rc<RefCell<Solid>>,
    dead_time: f64,
    remove_after: f64,
}

impl Killable {
    pub fn new(solid: Rc<RefCell<Solid>>) -> Self {
        let remove_after = 2.;
        let dead_time = 0.;
        Self {
            solid,
            remove_after,
            dead_time,
        }
    }
}

impl EntityTrait for Killable {
    fn name(&self) -> &str {
        "killable"
    }
    fn update(&mut self, entity: Rc<RefCell<Entity>>, dt: f64) {
        let dead = entity.borrow().living == Living::Dead;
        self.solid.borrow_mut().set_obstructs(!dead);
        if dead {
            self.dead_time += dt;
            if self.dead_time > self.remove_after {
                entity.borrow_mut().remove();
            }
        }
    }
}