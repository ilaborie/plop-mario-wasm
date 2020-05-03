use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::bullet_behavior::BulletBehavior;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::solid::Solid;
use crate::entity::traits::velocity::Velocity;
use crate::entity::{Entity, EntityFeature};
use crate::physics::Direction;
use core::cell::RefCell;
use std::rc::Rc;

pub struct BulletEntity {
    entity: Rc<RefCell<Entity>>,
}

impl BulletEntity {
    pub fn new(mut entity: Entity) -> Self {
        // Traits
        let solid = Rc::new(RefCell::new(Solid::new()));
        let behavior = BulletBehavior::default();
        let behavior = Rc::new(RefCell::new(behavior));
        let killable = Rc::new(RefCell::new(Killable::new(solid)));
        let velocity = Rc::new(RefCell::new(Velocity::default()));

        entity.traits.push(velocity);
        entity.traits.push(killable);
        entity.traits.push(behavior);

        // Note: need to check X/Y if we want to stop on wall (see Physics)

        // Features
        entity.features.push(EntityFeature::Killable);

        let entity = Rc::new(RefCell::new(entity));
        Self { entity }
    }
}

impl DrawableEntity for BulletEntity {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }

    fn entity_display(&self) -> Option<EntityDisplay> {
        let dx = self.entity.borrow().dx;
        let direction = if dx < 0. {
            Direction::Left
        } else {
            Direction::Right
        };

        let result = EntityDisplay::sprite(AnimationName::Bullet, Sprite::Bullet, direction);
        Some(result)
    }
}
