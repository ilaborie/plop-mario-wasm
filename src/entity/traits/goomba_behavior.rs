use crate::entity::traits::walk::Walk;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

pub struct GoombaBehavior {
    walk: Rc<RefCell<Walk>>,
    points: u32,
}

impl GoombaBehavior {
    pub fn new(walk: Rc<RefCell<Walk>>, points: u32) -> Self {
        Self { walk, points }
    }
}

impl EntityTrait for GoombaBehavior {
    fn name(&self) -> &str {
        "goomba"
    }
    fn collides(&mut self, us: Rc<RefCell<Entity>>, them: Rc<RefCell<Entity>>) {
        if them.borrow().living != Living::Alive {
            return;
        }
        if them.borrow().is_stomper() && them.borrow().living == Living::Alive {
            if them.borrow().dy > us.borrow().dy {
                // Dead
                us.borrow_mut().kill(them.borrow().id.as_str());
                them.borrow_mut().incr_score(self.points);
                self.walk.borrow_mut().disable();
            } else {
                // Kill
                them.borrow_mut().kill(us.borrow().id.as_str());
            }
        }
    }
}
