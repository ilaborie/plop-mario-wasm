use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::goomba_behavior::GoombaBehavior;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::solid::Solid;
use crate::entity::traits::walk::Walk;
use crate::entity::{Entity, EntityFeature, Living};
use crate::physics::Direction;
use core::cell::RefCell;
use std::rc::Rc;

pub struct GoombaEntity {
    entity: Rc<RefCell<Entity>>,
}

impl GoombaEntity {
    pub fn new(mut entity: Entity, physics: Physics) -> Self {
        // Traits
        let solid = Rc::new(RefCell::new(Solid::new()));
        let walk = Walk::new(entity.dx);
        let walk = Rc::new(RefCell::new(walk));
        let behavior = GoombaBehavior::default();
        let behavior = Rc::new(RefCell::new(behavior));
        let killable = Rc::new(RefCell::new(Killable::new(solid.clone(), 0., 0.)));
        let physics = Rc::new(RefCell::new(physics));

        entity.add_trait(solid);
        entity.add_trait(walk);
        entity.add_trait(behavior);
        entity.add_trait(killable);
        entity.add_trait(physics);

        // Features
        entity.features.push(EntityFeature::Killable);

        let entity = Rc::new(RefCell::new(entity));
        Self { entity }
    }
}

impl DrawableEntity for GoombaEntity {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }

    fn entity_display(&self) -> Option<EntityDisplay> {
        let dist = self.entity.borrow().lifetime();
        let dx = self.entity.borrow().dx;
        let direction = if dx < 0. {
            Direction::Left
        } else {
            Direction::Right
        };

        let result = if self.entity.borrow().living == Living::Dead {
            EntityDisplay::sprite(AnimationName::Walk, Sprite::Flat, direction)
        } else {
            EntityDisplay::animation(AnimationName::Walk, dist, direction)
        };
        Some(result)
    }
}
