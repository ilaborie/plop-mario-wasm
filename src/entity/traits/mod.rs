use crate::entity::{Entity, ObstructionSide};
use crate::physics::bounding_box::BoundingBox;
use std::cell::RefCell;
use std::rc::Rc;

pub mod go;
pub mod goomba_behavior;
pub mod jump;
pub mod killable;
pub mod koopa_behavior;
pub mod player_controller;
pub mod stomper;
pub mod walk;

pub trait EntityTrait {
    fn update(&mut self, _entity: Rc<RefCell<Entity>>, _dt: f64) {}
    fn obstruct(
        &mut self,
        _entity: Rc<RefCell<Entity>>,
        _side: ObstructionSide,
        _rect: BoundingBox,
    ) {
    }
    fn collides(&mut self, _us: Rc<RefCell<Entity>>, _them: Rc<RefCell<Entity>>) {}
}
