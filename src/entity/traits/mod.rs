use crate::entity::events::EventEmitter;
use crate::entity::{Entity, ObstructionSide};
use crate::game::GameContext;
use crate::physics::bounding_box::BBox;
use core::fmt;
use core::fmt::Formatter;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub mod go;
pub mod goomba_behavior;
pub mod jump;
pub mod killable;
pub mod koopa_behavior;
pub mod physics;
pub mod player_controller;
pub mod solid;
pub mod stomper;
pub mod walk;

pub trait EntityTrait {
    fn name(&self) -> &str;
    fn update(&mut self, _entity: Rc<RefCell<Entity>>, _context: &GameContext) {}
    fn obstruct(&mut self, _entity: Rc<RefCell<Entity>>, _side: ObstructionSide, _rect: BBox) {}
    fn collides(
        &mut self,
        _us: Rc<RefCell<Entity>>,
        _them: Rc<RefCell<Entity>>,
        _event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
    }
}

impl Debug for dyn EntityTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn update(entity: Rc<RefCell<Entity>>, context: &GameContext) {
    let traits = entity.clone().borrow().traits.clone();

    for t in traits.into_iter() {
        t.clone().borrow_mut().update(entity.clone(), context);
    }

    entity.borrow_mut().lifetime += context.dt();
}

pub fn obstruct(entity: Rc<RefCell<Entity>>, side: ObstructionSide, rect: BBox) {
    let mut traits = entity.clone().borrow().traits.clone();
    // NOTE: obstruct is called during the update of the Physics trait
    //       therefore, the Physics trait is borrowed and we cannot borrow it again
    //       so we remove it form the list trait
    traits.retain(|t| t.try_borrow().is_ok());

    for t in traits.iter() {
        t.borrow_mut().obstruct(entity.clone(), side, rect);
    }
}

pub fn collides(
    us: Rc<RefCell<Entity>>,
    them: Rc<RefCell<Entity>>,
    event_emitter: Rc<RefCell<EventEmitter>>,
) {
    let traits = us.borrow().traits.clone();

    for t in traits.iter() {
        t.borrow_mut()
            .collides(us.clone(), them.clone(), event_emitter.clone());
    }
}
