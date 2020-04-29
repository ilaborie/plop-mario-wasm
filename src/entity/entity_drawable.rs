use crate::entity::entity_display::EntityDisplay;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, EntityFeature, Living, ObstructionSide};
use crate::physics::bounding_box::BoundingBox;
use crate::physics::Size;
use std::cell::RefCell;
use std::rc::Rc;

pub type TraitUpdater = Box<dyn FnMut(Rc<RefCell<dyn EntityTrait>>) -> ()>;

pub trait DrawableEntity {
    fn id(&self) -> String {
        self.entity().borrow().id.clone()
    }
    fn living(&self) -> Living {
        self.entity().borrow().living
    }
    fn can_collide(&self) -> bool {
        self.entity().borrow().can_collide
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

    fn collision_box(&self) -> BoundingBox {
        self.entity().borrow().collision_box()
    }
    fn dx(&self) -> f64 {
        self.entity().borrow().dx
    }
    fn dy(&self) -> f64 {
        self.entity().borrow().dy
    }

    fn apply_velocity_x(&mut self, dt: f64) {
        self.entity().borrow_mut().apply_velocity_x(dt);
    }

    fn apply_velocity_y(&mut self, dt: f64) {
        self.entity().borrow_mut().apply_velocity_y(dt);
    }

    fn apply_gravity(&mut self, dt: f64) {
        self.entity().borrow_mut().apply_gravity(dt);
    }

    fn traits(&mut self, func: TraitUpdater);

    fn update(&mut self, dt: f64) {
        let entity = self.entity();
        entity.borrow_mut().update(dt);
        self.traits(Box::new(move |t| t.borrow_mut().update(entity.clone(), dt)));
    }
    fn obstruct(&mut self, side: ObstructionSide, rect: BoundingBox) {
        let entity = self.entity();
        self.traits(Box::new(move |t| {
            t.borrow_mut().obstruct(entity.clone(), side, rect)
        }));
    }
    fn collides(&mut self, entity: Rc<RefCell<dyn DrawableEntity>>) {
        let us = self.entity();
        let them = entity.borrow().entity();
        self.traits(Box::new(move |t| {
            t.borrow_mut().collides(us.clone(), them.clone())
        }))
    }
}
