use crate::entity::traits::EntityTrait;

pub mod animation;
pub mod sprite;
pub mod traits;

pub const ENTITY_SIZE: u32 = 64;

pub trait Updatable {
    fn update(&mut self, dt: f64);
}

#[derive(Default)]
pub struct Entity {
    traits: Vec<EntityTrait>,
}

impl Updatable for Entity {
    fn update(&mut self, dt: f64) {
        for t in self.traits.iter_mut() {
            t.update(dt);
        }
    }
}
