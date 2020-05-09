use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::events::EventBuffer;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Stomper {
    queue_bounce: bool,
}

impl Stomper {
    pub fn new() -> Self {
        let queue_bounce = false;
        Stomper { queue_bounce }
    }
}

impl EntityTrait for Stomper {
    fn name(&self) -> &str {
        "stomper"
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        if us.borrow().living != Living::Alive {
            return;
        }

        if them.borrow().is_killable() {
            let dy = us.borrow().dy;
            if dy > them.borrow().dy {
                event_buffer.borrow_mut().stomp(us, them);
                self.queue_bounce = true;
            }
        }
    }
}
