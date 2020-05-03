use crate::entity::events::EventEmitter;
use crate::entity::traits::walk::Walk;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

pub struct GoombaBehavior {
    walk: Rc<RefCell<Walk>>,
}

impl GoombaBehavior {
    pub fn new(walk: Rc<RefCell<Walk>>) -> Self {
        Self { walk }
    }
}

impl EntityTrait for GoombaBehavior {
    fn name(&self) -> &str {
        "goomba"
    }
    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        if them.borrow().living != Living::Alive {
            return;
        }
        if them.borrow().is_stomper() && them.borrow().living == Living::Alive {
            if them.borrow().dy > us.borrow().dy {
                // Dead
                event_emitter.borrow().kill(them, us);
                self.walk.borrow_mut().disable();
            } else {
                event_emitter.borrow().kill(us, them);
            }
        }
    }
}
