use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::jumping::Jumping;
use crate::physics::rectangle::BoundingBox;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Jump {
    jumping: Rc<RefCell<Jumping>>,
}

impl Jump {
    pub fn new(jumping: Rc<RefCell<Jumping>>) -> Self {
        Self { jumping }
    }
}

impl EntityTrait for Jump {
    fn update(&mut self, entity: &mut Entity, dt: f64) {
        if self.jumping.borrow().request_time() > 0. {
            self.jumping.borrow_mut().engage(dt);
        }
        if self.jumping.borrow().engage_time() > 0. {
            let dx = entity.dx;
            let speed_bonus = dx.abs() * self.jumping.borrow().speed_boost();
            let dy = -(self.jumping.borrow().velocity() + speed_bonus) * dt;
            entity.dy = dy;
            self.jumping.borrow_mut().decr_engage_time(dt);
        }
    }

    fn obstruct(&mut self, _entity: &mut Entity, side: ObstructionSide, _rect: &BoundingBox) {
        match side {
            ObstructionSide::Bottom =>
            // FIXME may set heading
            {
                self.jumping.borrow_mut().set_ready()
            }
            ObstructionSide::Top =>
            // Aie
            {
                self.jumping.borrow_mut().cancel()
            }
            _ => {}
        }
    }
}
