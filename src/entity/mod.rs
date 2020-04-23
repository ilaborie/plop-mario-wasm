use crate::entity::traits::EntityTrait;

pub mod sprite;
pub mod traits;

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
