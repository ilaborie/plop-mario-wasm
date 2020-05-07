use crate::entity::traits::solid::Solid;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::game::GameContext;
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

    fn on_killed(&mut self, entity: Rc<RefCell<Entity>>) {
        entity.borrow_mut().living = Living::Dead;
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext) {
        let alive = entity.borrow().living == Living::Alive;
        let dead = entity.borrow().living == Living::Dead;
        self.solid.borrow_mut().set_obstructs(alive);
        if dead {
            self.dead_time += context.dt();
            if self.dead_time > self.remove_after {
                entity.borrow_mut().remove();
            }
        }
    }
}
