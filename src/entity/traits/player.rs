use crate::audio::sounds::Fx;
use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const COIN_LIVE_THRESHOLD: u32 = 100;

#[derive(Default)]
pub struct PlayerTrait {
    lives: Rc<Cell<u32>>,
    coins: Rc<Cell<u32>>,
    score: Rc<Cell<u32>>,
}

impl PlayerTrait {
    pub fn new(lives: u32) -> Self {
        let lives = Rc::new(Cell::new(lives));
        let coins = Rc::default();
        let score = Rc::default();

        Self {
            lives,
            coins,
            score,
        }
    }

    pub fn lives(&self) -> Rc<Cell<u32>> {
        self.lives.clone()
    }
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

    fn on_killed(&mut self, _entity: Rc<RefCell<Entity>>) {
        let lives = self.lives.get();
        self.lives.set(lives - 1);
    }

    fn on_coin(&mut self, entity: Rc<RefCell<Entity>>, count: u32) {
        entity.borrow_mut().play_fx(Fx::Coin);
        let mut coin = self.coins.get() + count;
        if coin >= COIN_LIVE_THRESHOLD {
            let lives = self.lives.get() + coin / COIN_LIVE_THRESHOLD;
            self.lives.set(lives);
            coin %= COIN_LIVE_THRESHOLD;
        }
        self.coins.set(coin);
    }
}
