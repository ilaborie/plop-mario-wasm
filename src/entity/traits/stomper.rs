use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
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
    fn update(&mut self, entity: Rc<RefCell<Entity>>, dt: f64) {
        if self.queue_bounce {
            self.queue_bounce = false;
            entity.borrow_mut().dy -= self.bounce_speed * dt;
        }
    }

    fn collides(&mut self, us: Rc<RefCell<Entity>>, them: Rc<RefCell<Entity>>) {
        let killable = them.borrow().is_killable();
        if killable {
            let dy = us.borrow().dy;
            if dy > them.borrow().dy {
                let top = them.borrow().collision_box().top();
                let height = us.borrow().size.height as f64;
                us.borrow_mut().y = top - height;
                self.queue_bounce = true;
            }
        }
    }
}
