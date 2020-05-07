use crate::entity::events::EventBuffer;
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

    fn on_killed(&mut self, entity: Rc<RefCell<Entity>>) {
        self.walk.borrow_mut().disable();
        entity.borrow_mut().dx = 0.;
        entity.borrow_mut().dy = 0.;
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        if them.borrow().is_stomper() && us.borrow().living == Living::Alive {
            if them.borrow().dy > us.borrow().dy {
                // Dead
                us.borrow_mut().living = Living::Dead;
                event_buffer.borrow_mut().kill(them, us);
            } else {
                event_buffer.borrow_mut().kill(us, them);
            }
        }
    }
}
