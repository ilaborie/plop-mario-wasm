use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Default)]
pub struct PlayerTrait {
    coins: Rc<Cell<u32>>,
    score: Rc<Cell<u32>>,
}

impl PlayerTrait {
    pub fn score(&self) -> Rc<Cell<u32>> {
        self.score.clone()
    }

    pub fn coins(&self) -> Rc<Cell<u32>> {
        self.coins.clone()
    }
}

impl EntityTrait for PlayerTrait {
    fn name(&self) -> &str {
        "player"
    }

    fn on_stomper(&mut self, _entity: Rc<RefCell<Entity>>) {
        let sc = self.score.get();
        self.score.set(sc + 20);
    }

    fn on_killer(&mut self, _entity: Rc<RefCell<Entity>>) {
        let sc = self.score.get();
        self.score.set(sc + 100);
    }
}
