use crate::entity::events::EventEmitter;
use crate::entity::traits::collides;
use crate::entity::Entity;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct EntityCollider {
    entities: Vec<Rc<RefCell<Entity>>>,
}

impl EntityCollider {
    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }
    pub fn check(&self, subject: Rc<RefCell<Entity>>, event_emitter: Rc<RefCell<EventEmitter>>) {
        let subject_box = subject.borrow().collision_box();

        for entity in self.entities.iter() {
            if entity.borrow().id == subject.borrow().id {
                continue;
            }
            let entity_box = entity.borrow().collision_box();

            if subject_box.overlaps(entity_box) {
                // collision
                collides(subject.clone(), entity.clone(), event_emitter.clone());
                collides(entity.clone(), subject.clone(), event_emitter.clone());
            }
        }
    }
}
