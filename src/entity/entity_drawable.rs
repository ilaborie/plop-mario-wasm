use crate::entity::entity_display::EntityDisplay;
use crate::entity::{Entity, EntityFeature, Living};
use crate::physics::Size;
use std::cell::RefCell;
use std::rc::Rc;

pub trait DrawableEntity {
    fn id(&self) -> String {
        self.entity().borrow().id.clone()
    }
    fn living(&self) -> Living {
        self.entity().borrow().living
    }
    fn is(&self, feature: EntityFeature) -> bool {
        self.entity().borrow().features.contains(&feature)
    }
    fn entity(&self) -> Rc<RefCell<Entity>>;
    fn entity_display(&self) -> EntityDisplay;

    fn position(&self) -> (f64, f64) {
        self.entity().borrow().position()
    }

    fn size(&self) -> Size {
        self.entity().borrow().size()
    }
}
