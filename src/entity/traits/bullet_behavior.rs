use core::cell::RefCell;
use std::rc::Rc;

use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::events::EventBuffer;

#[derive(Default)]
pub struct BulletBehavior {}

impl EntityTrait for BulletBehavior {
    fn name(&self) -> &str {
        "bullet"
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        if !them.borrow().is_stomper() || us.borrow().living != Living::Alive {
            return;
        }

        // log(&format!("Bullet Collides {:?} <-> {:?}", us, them));
        if them.borrow().dy > us.borrow().dy {
            event_buffer
                .borrow_mut()
                .kill(them.borrow().id(), us.borrow().id());
        } else {
            event_buffer
                .borrow_mut()
                .kill(us.borrow().id(), them.borrow().id());
        }
    }
}
