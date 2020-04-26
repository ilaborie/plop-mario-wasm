use crate::entity::traits::EntityTrait;

pub mod animation;
pub mod sprite;
pub mod traits;

pub const ENTITY_SIZE: u32 = 64;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObstructionSide {
    Top,
    Right,
    Bottom,
    Left,
}

pub trait Updatable {
    fn update(&mut self, dt: f64);
}

#[derive(Default)]
pub struct Entity {
    traits: Vec<EntityTrait>,
}

impl Entity {
    pub fn obstruct(&mut self, side: ObstructionSide) {
        for t in self.traits.iter_mut() {
            t.obstruct(side);
        }
    }
}

impl Updatable for Entity {
    fn update(&mut self, dt: f64) {
        for t in self.traits.iter_mut() {
            t.update(dt);
        }
    }
}
