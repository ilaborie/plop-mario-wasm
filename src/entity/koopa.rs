use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::{DrawableEntity, TraitUpdater};
use crate::entity::traits::koopa_behavior::{KoopaBehavior, KoopaState};
use crate::entity::traits::walk::Walk;
use crate::entity::{Entity, EntityFeature};
use crate::physics::Direction;
use crate::utils::log;
use core::cell::RefCell;
use std::rc::Rc;

pub struct KoopaEntity {
    entity: Rc<RefCell<Entity>>,
    walk: Rc<RefCell<Walk>>,
    behavior: Rc<RefCell<KoopaBehavior>>,
}

impl KoopaEntity {
    pub fn new(entity: Entity) -> Self {
        let entity = Rc::new(RefCell::new(entity));

        // Traits
        let walk = Walk::new(entity.borrow().dx);
        let walk = Rc::new(RefCell::new(walk));
        let behavior = KoopaBehavior::new(walk.clone());
        let behavior = Rc::new(RefCell::new(behavior));

        // Features
        entity.borrow_mut().features.push(EntityFeature::Killable);

        Self {
            entity,
            walk,
            behavior,
        }
    }
}

impl DrawableEntity for KoopaEntity {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }

    fn entity_display(&self) -> EntityDisplay {
        let dist = self.entity.borrow().lifetime();
        let dx = self.entity.borrow().dx;
        let direction = if dx < 0. {
            Direction::Left
        } else {
            Direction::Right
        };

        let state = self.behavior.borrow().state();
        log(&format!("Koopa {} in {:?}", self.entity.borrow().id, state));
        match state {
            KoopaState::Hiding => {
                if self.behavior.borrow().hide_time() > 3. {
                    EntityDisplay::animation(AnimationName::Wake, dist, direction)
                } else {
                    EntityDisplay::sprite(AnimationName::Walk, Sprite::Hiding, direction)
                }
            }
            KoopaState::Panic => {
                EntityDisplay::sprite(AnimationName::Walk, Sprite::Hiding, direction)
            }
            KoopaState::Walking => EntityDisplay::animation(AnimationName::Walk, dist, direction),
        }
    }

    fn traits(&mut self, mut func: TraitUpdater) {
        func(self.walk.clone());
        func(self.behavior.clone());
    }
}
