use core::cell::RefCell;
use std::rc::Rc;

use crate::assets::levels::TriggerDefinition;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::trigger::TriggerTrait;
use crate::entity::Entity;
use crate::physics::bounding_box::BBox;
use crate::physics::Size;

pub struct TriggerEntity {
    entity: Rc<RefCell<Entity>>,
}

impl TriggerEntity {
    pub fn new(trigger: TriggerDefinition) -> Self {
        let id = String::from("Trigger");
        let size = Size::new(64, 64);
        let (x, y) = trigger.position();
        let bbox = BBox::new(x as f64, y as f64, size);
        let mut entity = Entity::new(id, bbox, size, None);

        // Traits
        let trigger_trait = Rc::new(RefCell::new(TriggerTrait::new(trigger)));
        entity.add_trait(trigger_trait);

        let entity = Rc::new(RefCell::new(entity));
        Self { entity }
    }
}

impl DrawableEntity for TriggerEntity {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }
}
