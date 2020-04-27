use crate::entity::{Entity, ObstructionSide};
use crate::physics::rectangle::BoundingBox;

pub mod go;
pub mod jump;
pub mod walk;

pub trait EntityTrait {
    fn update(&mut self, entity: &mut Entity, dt: f64);
    fn obstruct(&mut self, entity: &mut Entity, side: ObstructionSide, rect: &BoundingBox);
}
