use crate::entity::entity_drawable::DrawableEntity;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct EntityCollider {
    entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
}

impl EntityCollider {
    pub fn add_entity(&mut self, entity: Rc<RefCell<dyn DrawableEntity>>) {
        self.entities.push(entity);
    }
    pub fn check(&self, subject: Rc<RefCell<dyn DrawableEntity>>) {
        let subject_box = subject.borrow().collision_box();

        for entity in self.entities.iter() {
            if entity.borrow().id() == subject.borrow().id() {
                continue;
            }
            let entity_box = entity.borrow().collision_box();

            if subject_box.overlaps(entity_box) {
                // collision
                subject.borrow_mut().collides(entity.clone());
                entity.borrow_mut().collides(subject.clone());
            }
        }
    }
}
