use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::physics::Position;
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

pub struct PlayerController {
    checkpoint: Rc<RefCell<Position>>,
    player: Rc<RefCell<Entity>>,
    time: Rc<Cell<f64>>,
    removed_time: f64,
    respawn_after: f64,
}

impl PlayerController {
    pub fn new(
        player: Rc<RefCell<Entity>>,
        time: Rc<Cell<f64>>,
        checkpoint: Rc<RefCell<Position>>,
    ) -> Self {
        let respawn_after = 3.;
        let removed_time = 0.;
        Self {
            checkpoint,
            player,
            time,
            removed_time,
            respawn_after,
        }
    }
}

impl EntityTrait for PlayerController {
    fn name(&self) -> &str {
        "player_controller"
    }

    fn update(&mut self, _entity: Rc<RefCell<Entity>>, dt: f64) {
        let living = self.player.borrow().living;
        match living {
            Living::Alive => {
                let time_left = self.time.get() as f64 - dt * 2.;
                self.time.set(time_left);
            }
            Living::NoExistence => {
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
            _ => {}
        }
    }
}
