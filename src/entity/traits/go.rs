use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::motion::{Direction, Motion};
use crate::physics::rectangle::BoundingBox;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Go {
    motion: Rc<RefCell<Motion>>,
}

impl Go {
    pub fn new(motion: Rc<RefCell<Motion>>) -> Self {
        Self { motion }
    }
}

impl EntityTrait for Go {
    fn update(&mut self, entity: &mut Entity, dt: f64) {
        let direction = self.motion.borrow().direction;
        let factor = match direction {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
            Direction::Stop => 0.0,
        };
        let dx = entity.dx;
        let abs_dx = dx.abs();

        if direction != Direction::Stop {
            let ddx = factor * self.motion.borrow().acceleration * dt;
            entity.dx += ddx;
        } else if dx != 0. {
            let ddx = abs_dx.min(self.motion.borrow().deceleration_base);
            let factor = if dx > 0. { -1. } else { 1. };
            entity.dx += factor * ddx;
        } else {
            self.motion.borrow_mut().distance = 0.;
        }

        // Drag
        let dx = entity.dx;
        let drag = self.motion.borrow().drag_factor / 1000. * dx * abs_dx;
        entity.dx -= drag;

        // log(&format!("dx {}",  dx).to_string());
        self.motion.borrow_mut().distance += dt * abs_dx;
    }

    fn obstruct(&mut self, entity: &mut Entity, side: ObstructionSide, rect: &BoundingBox) {
        match side {
            ObstructionSide::Right => {
                let width = entity.size.width as f64;
                let x = rect.left() - width;
                entity.set_x(x, 0.);
            }
            ObstructionSide::Left => {
                let x = rect.right();
                entity.set_x(x, 0.);
            }
            ObstructionSide::Top => {
                let y = rect.bottom();
                entity.set_y(y, 0.);
            }
            ObstructionSide::Bottom => {
                let height = entity.size.height as f64;
                let y = rect.top() - height;
                entity.set_y(y, 0.);
            }
        }
    }
}
