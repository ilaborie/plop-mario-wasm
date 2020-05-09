use crate::entity::entity_display::EntityDisplay;
use crate::entity::{Entity, EntityFeature, Living};
use crate::physics::Size;
use core::fmt;
use core::fmt::{Debug, Formatter};
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
    fn entity_display(&self) -> Option<EntityDisplay> {
        None
    }

    fn position(&self) -> (f64, f64) {
        self.entity().borrow().position()
    }

    fn size(&self) -> Size {
        self.entity().borrow().size()
    }
}

impl Debug for dyn DrawableEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
