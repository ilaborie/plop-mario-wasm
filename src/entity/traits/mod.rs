use core::fmt;
use core::fmt::Formatter;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::entity::{Entity, ObstructionSide};
use crate::events::EventBuffer;
use crate::game::GameContext;
use crate::physics::bounding_box::BBox;
use crate::scene::level::Level;

pub mod bullet_behavior;
pub mod emitter;
pub mod go;
pub mod goomba_behavior;
pub mod gravity;
pub mod jump;
pub mod killable;
pub mod koopa_behavior;
pub mod level_timer;
pub mod physics;
pub mod player;
pub mod player_controller;
pub mod solid;
pub mod stomper;
pub mod trigger;
pub mod velocity;
pub mod walk;

pub trait EntityTrait {
    fn name(&self) -> &str;

    // Events
    fn on_stomper(&mut self, _entity: Rc<RefCell<Entity>>) {}
    fn on_stomped(&mut self, _entity: Rc<RefCell<Entity>>) {}
    fn on_killer(&mut self, _entity: Rc<RefCell<Entity>>) {}
    fn on_killed(&mut self, _entity: Rc<RefCell<Entity>>) {}
    fn on_coin(&mut self, _entity: Rc<RefCell<Entity>>, _count: u32) {}

    // Operations
    fn update(&mut self, _entity: Rc<RefCell<Entity>>, _context: &GameContext, _level: &Level) {}
    fn obstruct(&mut self, _entity: Rc<RefCell<Entity>>, _side: ObstructionSide, _rect: BBox) {}
    fn collides(
        &mut self,
        _us: Rc<RefCell<Entity>>,
        _them: Rc<RefCell<Entity>>,
        _event_emitter: Rc<RefCell<EventBuffer>>,
    ) {
    }
}

impl Debug for dyn EntityTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn update(entity: Rc<RefCell<Entity>>, context: &GameContext, level: &Level) {
    let traits = entity.borrow().traits.clone();

    for t in traits.into_iter() {
        t.clone()
            .borrow_mut()
            .update(entity.clone(), context, level);
    }

    entity.borrow_mut().lifetime += context.dt();
}

pub fn obstruct(entity: Rc<RefCell<Entity>>, side: ObstructionSide, rect: BBox) {
    let mut traits = entity.borrow().traits.clone();
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
    event_buffer: Rc<RefCell<EventBuffer>>,
) {
    let traits = us.borrow().traits.clone();

    for t in traits.iter() {
        t.borrow_mut()
            .collides(us.clone(), them.clone(), event_buffer.clone());
    }
}
