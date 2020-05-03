use crate::audio::player::Fx;
use crate::entity::events::EventEmitter;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::game::GameContext;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Stomper {
    bounce_speed: f64,
    queue_bounce: bool,
}

impl Stomper {
    pub fn new(bounce_speed: f64) -> Self {
        let queue_bounce = false;
        Stomper {
            bounce_speed,
            queue_bounce,
        }
    }
}

impl EntityTrait for Stomper {
    fn name(&self) -> &str {
        "stomper"
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext) {
        if self.queue_bounce {
            entity.borrow_mut().play_fx(Fx::Stomp);
            self.queue_bounce = false;
            entity.borrow_mut().dy -= self.bounce_speed * context.dt();
        }
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        if us.borrow().living != Living::Alive {
            return;
        }

        let killable = them.borrow().is_killable();
        if killable {
            let dy = us.borrow().dy;
            if dy > them.borrow().dy {
                let top = them.borrow().collision_box().top();
                let height = us.borrow().size.height as f64;
                us.borrow_mut().y = top - height;
                event_emitter.borrow().stomp(us, them);
                self.queue_bounce = true;
            }
        }
    }
}
