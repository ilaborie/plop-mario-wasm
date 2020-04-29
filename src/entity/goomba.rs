use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::{DrawableEntity, TraitUpdater};
use crate::entity::traits::goomba_behavior::GoombaBehavior;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::walk::Walk;
use crate::entity::{Entity, EntityFeature, Living};
use crate::physics::Direction;
use core::cell::RefCell;
use std::rc::Rc;

pub struct GoombaEntity {
    entity: Rc<RefCell<Entity>>,
    walk: Rc<RefCell<Walk>>,
    behavior: Rc<RefCell<GoombaBehavior>>,
    killable: Rc<RefCell<Killable>>,
}

impl GoombaEntity {
    pub fn new(entity: Entity) -> Self {
        let entity = Rc::new(RefCell::new(entity));

        // Traits
        let walk = Walk::new(entity.borrow().dx);
        let walk = Rc::new(RefCell::new(walk));
        let behavior = GoombaBehavior::new(walk.clone());
        let behavior = Rc::new(RefCell::new(behavior));
        let killable = Rc::default();

        // Features
        entity.borrow_mut().features.push(EntityFeature::Killable);

        Self {
            entity,
            walk,
            behavior,
            killable,
        }
    }
}

impl DrawableEntity for GoombaEntity {
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

        if self.entity.borrow().living == Living::Dead {
            EntityDisplay::sprite(AnimationName::Walk, Sprite::Flat, direction)
        } else {
            EntityDisplay::animation(AnimationName::Walk, dist, direction)
        }
    }

    fn traits(&mut self, mut func: TraitUpdater) {
        func(self.walk.clone());
        func(self.behavior.clone());
        func(self.killable.clone());
    }
}
