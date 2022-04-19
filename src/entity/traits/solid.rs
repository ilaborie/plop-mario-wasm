use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::bounding_box::BBox;

pub struct Solid {
    obstructs: bool,
}

impl Solid {
    pub fn new() -> Self {
        let obstructs = true;
        Self { obstructs }
    }

    pub fn set_obstructs(&mut self, obstructs: bool) {
        self.obstructs = obstructs;
    }
}

impl EntityTrait for Solid {
    fn name(&self) -> &str {
        "solid"
    }

    fn on_killed(&mut self, _entity: Rc<RefCell<Entity>>) {
        self.obstructs = false;
    }

    fn obstruct(&mut self, entity: Rc<RefCell<Entity>>, side: ObstructionSide, rect: BBox) {
        if !self.obstructs {
            return;
        }

        match side {
            ObstructionSide::Right => {
                let width = entity.borrow().size.width as f64;
                let x = rect.left() - width;
                entity.borrow_mut().set_x(x, 0.);
            }
            ObstructionSide::Left => {
                let x = rect.right();
                entity.borrow_mut().set_x(x, 0.);
            }
            ObstructionSide::Top => {
                let y = rect.bottom();
                entity.borrow_mut().set_y(y, 0.);
            }
            ObstructionSide::Bottom => {
                let height = entity.borrow().size.height as f64;
                let y = rect.top() - height;
                entity.borrow_mut().set_y(y, 0.);
            }
        }
    }
}
