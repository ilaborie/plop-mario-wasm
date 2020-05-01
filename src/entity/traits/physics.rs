use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::physics::tile_collider::TileCollider;
use crate::physics::GravityForce;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Physics {
    gravity: GravityForce,
    tile_collider: Rc<TileCollider>,
}

impl Physics {
    pub fn new(gravity: GravityForce, tile_collider: Rc<TileCollider>) -> Self {
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

    fn update(&mut self, entity: Rc<RefCell<Entity>>, dt: f64) {
        // Y
        entity.borrow_mut().apply_velocity_y(dt);
        self.tile_collider.check_y(entity.clone());

        // X
        entity.borrow_mut().apply_velocity_x(dt);
        self.tile_collider.check_x(entity.clone());

        // Gravity
        entity.borrow_mut().apply_gravity(self.gravity.g * dt);
    }
}
