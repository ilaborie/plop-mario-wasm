use core::cell::RefCell;
use std::rc::Rc;

use crate::assets::animations::AnimationName;
use crate::assets::sprites::Sprite;
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::koopa_behavior::{KoopaBehavior, KoopaState};
use crate::entity::traits::physics::Physics;
use crate::entity::traits::solid::Solid;
use crate::entity::traits::walk::Walk;
use crate::entity::{Entity, EntityFeature};
use crate::physics::Direction;

pub struct KoopaEntity {
    entity: Rc<RefCell<Entity>>,
    behavior: Rc<RefCell<KoopaBehavior>>,
}

impl KoopaEntity {
    pub fn new(mut entity: Entity, physics: Physics) -> Self {
        // Traits
        let solid = Rc::new(RefCell::new(Solid::new()));
        let walk = Walk::new(entity.dx);
        let walk = Rc::new(RefCell::new(walk));
        let behavior = KoopaBehavior::new(walk.clone());
        let behavior = Rc::new(RefCell::new(behavior));
        let killable = Rc::new(RefCell::new(Killable::new(solid.clone(), 100., -200.)));
        let physics = Rc::new(RefCell::new(physics));

        entity.add_trait(solid);
        entity.add_trait(walk);
        entity.add_trait(behavior.clone());
        entity.add_trait(killable);
        entity.add_trait(physics);

        // Features
        entity.features.push(EntityFeature::Killable);

        let entity = Rc::new(RefCell::new(entity));
        Self { entity, behavior }
    }
}

impl DrawableEntity for KoopaEntity {
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

        let state = self.behavior.borrow().state();
        let result = match state {
            KoopaState::Hiding => {
                if self.behavior.borrow().hide_time() > 3. {
                    EntityDisplay::animation(AnimationName::Wake, dist, direction)
                } else {
                    EntityDisplay::sprite_direction(AnimationName::Walk, Sprite::Hiding, direction)
                }
            }
            KoopaState::Panic => {
                EntityDisplay::sprite_direction(AnimationName::Walk, Sprite::Hiding, direction)
            }
            KoopaState::Walking => EntityDisplay::animation(AnimationName::Walk, dist, direction),
        };

        Some(result)
    }
}
