use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::game::GameContext;
use crate::physics::tile_collider::TileCollider;
use crate::physics::GravityForce;
use crate::scene::level::Level;

pub struct Physics {
    gravity: GravityForce,
    tile_collider: Rc<RefCell<TileCollider>>,
}

impl Physics {
    pub fn new(gravity: GravityForce, tile_collider: Rc<RefCell<TileCollider>>) -> Self {
        Self {
            gravity,
            tile_collider,
        }
    }
}

impl EntityTrait for Physics {
    fn name(&self) -> &str {
        "physics"
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext, _level: &Level) {
        let dt = context.dt();

        // Y
        entity.borrow_mut().apply_velocity_y(dt);
        self.tile_collider
            .borrow_mut()
            .check_y(entity.clone(), context.emitter());

        // X
        entity.borrow_mut().apply_velocity_x(dt);
        self.tile_collider
            .borrow_mut()
            .check_x(entity.clone(), context.emitter());

        // Gravity
        entity.borrow_mut().apply_gravity(self.gravity.g * dt);
    }
}
