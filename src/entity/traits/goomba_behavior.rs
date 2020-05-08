use crate::entity::events::EventBuffer;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GoombaBehavior {}

impl EntityTrait for GoombaBehavior {
    fn name(&self) -> &str {
        "goomba"
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        if them.borrow().is_stomper() && us.borrow().living == Living::Alive {
            let us_id = us.borrow().id();
            let them_id = them.borrow().id();
            if them.borrow().dy > us.borrow().dy {
                event_buffer.borrow_mut().kill(them_id, us_id);
            } else {
                event_buffer.borrow_mut().kill(us_id, them_id);
            }
        }
    }
}
