use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::game::GameContext;
use crate::scene::level::Level;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Velocity {}

impl EntityTrait for Velocity {
    fn name(&self) -> &str {
        "velocity"
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext, _level: &Level) {
        let dt = context.dt();
        entity.borrow_mut().apply_velocity_y(dt);
        entity.borrow_mut().apply_velocity_x(dt);
    }
}
