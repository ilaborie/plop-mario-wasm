use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::game::GameContext;
use crate::physics::bounding_box::BBox;
use crate::scene::level::Level;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Walk {
    speed: f64,
    enable: bool,
}

impl Walk {
    pub fn new(speed: f64) -> Self {
        let enable = true;
        Self { speed, enable }
    }
    pub fn speed(&self) -> f64 {
        self.speed
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
    pub fn enable(&mut self) {
        self.enable = true;
    }
    pub fn disable(&mut self) {
        self.enable = false;
    }
}

impl EntityTrait for Walk {
    fn name(&self) -> &str {
        "walk"
    }

    fn on_stomped(&mut self, _entity: Rc<RefCell<Entity>>) {
        self.enable = false;
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, _context: &GameContext, _level: &Level) {
        // Move X
        if self.enable {
            entity.borrow_mut().dx = self.speed;
        }
    }

    fn obstruct(&mut self, _entity: Rc<RefCell<Entity>>, side: ObstructionSide, _rect: BBox) {
        match side {
            ObstructionSide::Right | ObstructionSide::Left => {
                self.speed *= -1.;
            }
            _ => {}
        }
    }
}
