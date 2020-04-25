use crate::physics::jumping::Jumping;
use crate::physics::motion::{Direction, Motion};
use crate::physics::velocity::Velocity;
use std::cell::RefCell;
use std::rc::Rc;

pub enum EntityTrait {
    Jump {
        velocity: Rc<RefCell<Velocity>>,
        jumping: Rc<RefCell<Jumping>>,
    },
    Go {
        velocity: Rc<RefCell<Velocity>>,
        go: Rc<RefCell<Motion>>,
    },
}

impl EntityTrait {
    pub fn jump(velocity: Rc<RefCell<Velocity>>, jumping: Rc<RefCell<Jumping>>) -> EntityTrait {
        EntityTrait::Jump { velocity, jumping }
    }

    pub fn go(velocity: Rc<RefCell<Velocity>>, go: Rc<RefCell<Motion>>) -> EntityTrait {
        EntityTrait::Go { velocity, go }
    }

    pub fn update(&mut self, dt: f64) {
        match self {
            EntityTrait::Jump { velocity, jumping } => {
                if jumping.borrow().engage_time() > 0.0 {
                    velocity
                        .borrow_mut()
                        .incr_dy(-jumping.borrow().velocity() * dt);
                    jumping.borrow_mut().decr_engage_time(dt);
                }
            }
            EntityTrait::Go { velocity, go } => {
                let factor = match go.borrow().direction {
                    Direction::Right => 1.0,
                    Direction::Left => -1.0,
                    Direction::Stop => 0.0,
                };
                velocity
                    .borrow_mut()
                    .set_dx(factor * go.borrow().speed * dt);
            }
        }
    }
}
