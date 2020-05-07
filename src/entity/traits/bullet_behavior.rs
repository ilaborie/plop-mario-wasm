use crate::entity::events::EventBuffer;
use crate::entity::traits::solid::Solid;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use core::cell::RefCell;
use std::rc::Rc;

pub struct BulletBehavior {
    solid: Rc<RefCell<Solid>>,
}

impl BulletBehavior {
    pub fn new(solid: Rc<RefCell<Solid>>) -> Self {
        Self { solid }
    }
}

impl EntityTrait for BulletBehavior {
    fn name(&self) -> &str {
        "bullet"
    }

    fn on_killed(&mut self, entity: Rc<RefCell<Entity>>) {
        self.solid.borrow_mut().set_obstructs(false);
        entity.borrow_mut().dy += 200.;
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventBuffer>>,
    ) {
        if !them.borrow().is_stomper() {
            return;
        }

        if them.borrow().dy > us.borrow().dy && us.borrow().living == Living::Alive {
            us.borrow_mut().living = Living::Dead;
            event_emitter.borrow_mut().kill(them, us);
        } else if them.borrow().is_killable() {
            event_emitter.borrow_mut().kill(us, them);
        }
    }
}
