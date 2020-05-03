use crate::entity::events::EventEmitter;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

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
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        if them.borrow().living != Living::Alive {
            return;
        }
        if them.borrow().is_stomper() && them.borrow().living == Living::Alive {
            if them.borrow().dy > us.borrow().dy {
                // Dead
                event_emitter.borrow().kill(them, us, 0., 200.);
            } else {
                event_emitter.borrow().kill(us, them, 0., -300.);
            }
        }
    }
}
