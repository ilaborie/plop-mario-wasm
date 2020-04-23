use crate::entity::player::PlayerEntity;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct PlayerEntityLayer {
    player: Rc<RefCell<PlayerEntity>>,
}

impl PlayerEntityLayer {
    pub(crate) fn new(player: Rc<RefCell<PlayerEntity>>) -> Self {
        Self { player }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.player.borrow().draw(context);
    }
}
