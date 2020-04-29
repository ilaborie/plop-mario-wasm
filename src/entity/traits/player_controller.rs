use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::physics::Position;
use core::cell::RefCell;
use std::rc::Rc;

pub struct PlayerController {
    checkpoint: Rc<RefCell<Position>>,
    player: Rc<RefCell<Entity>>,
    removed_time: f64,
    respawn_after: f64,
}

impl PlayerController {
    pub fn new(player: Rc<RefCell<Entity>>, checkpoint: Rc<RefCell<Position>>) -> Self {
        let respawn_after = 3.;
        let removed_time = 0.;
        Self {
            checkpoint,
            player,
            removed_time,
            respawn_after,
        }
    }
}

impl EntityTrait for PlayerController {
    fn update(&mut self, _entity: Rc<RefCell<Entity>>, dt: f64) {
        if self.player.borrow().living == Living::NoExistence {
            self.removed_time += dt;
            if self.removed_time > self.respawn_after {
                self.removed_time = 0.;
                let x = self.checkpoint.borrow().x();
                let y = self.checkpoint.borrow().y();
                self.player.borrow_mut().set_x(x, 0.);
                self.player.borrow_mut().set_y(y, 0.);
                self.player.borrow_mut().revive();
            }
        }
    }
}
