use crate::entity::ObstructionSide;
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
        motion: Rc<RefCell<Motion>>,
    },
}

impl EntityTrait {
    pub fn jump(velocity: Rc<RefCell<Velocity>>, jumping: Rc<RefCell<Jumping>>) -> EntityTrait {
        EntityTrait::Jump { velocity, jumping }
    }

    pub fn go(velocity: Rc<RefCell<Velocity>>, motion: Rc<RefCell<Motion>>) -> EntityTrait {
        EntityTrait::Go { velocity, motion }
    }

    pub fn update(&mut self, dt: f64) {
        match self {
            // Jumping
            EntityTrait::Jump { velocity, jumping } => {
                if jumping.borrow().request_time() > 0. {
                    jumping.borrow_mut().engage(dt);
                }
                if jumping.borrow().engage_time() > 0. {
                    let speed_bonus = velocity.borrow().dx().abs() * jumping.borrow().speed_boost();
                    let dy = -(jumping.borrow().velocity() + speed_bonus) * dt;
                    velocity.borrow_mut().incr_dy(dy);
                    jumping.borrow_mut().decr_engage_time(dt);
                }
            }
            // Moving
            EntityTrait::Go { velocity, motion } => {
                let direction = motion.borrow().direction;
                let factor = match direction {
                    Direction::Right => 1.0,
                    Direction::Left => -1.0,
                    Direction::Stop => 0.0,
                };
                let dx = velocity.borrow_mut().dx();
                let abs_dx = dx.abs();

                if direction != Direction::Stop {
                    let ddx = factor * motion.borrow().acceleration * dt;
                    velocity.borrow_mut().incr_dx(ddx);
                } else if dx != 0. {
                    let ddx = abs_dx.min(motion.borrow().deceleration_base);
                    let factor = if dx > 0. { -1. } else { 1. };
                    velocity.borrow_mut().incr_dx(factor * ddx);
                } else {
                    motion.borrow_mut().distance = 0.;
                }

                let dx = velocity.borrow().dx();
                let drag = motion.borrow().drag_factor / 1000. * dx * abs_dx;
                velocity.borrow_mut().incr_dx(-drag);

                motion.borrow_mut().distance += dt * abs_dx;
            }
        }
    }

    pub fn obstruct(&mut self, side: ObstructionSide) {
        match self {
            EntityTrait::Jump {
                velocity: _,
                jumping,
            } => match side {
                ObstructionSide::Bottom => jumping.borrow_mut().set_ready(),
                ObstructionSide::Top => jumping.borrow_mut().cancel(),
                _ => {}
            },
            _ => {}
        }
    }
}
