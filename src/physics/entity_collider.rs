use crate::entity::events::EventBuffer;
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
        // log(&format!("Add entity: {:?}", entity));
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, id: &str) {
        // log(&format!("Remove entity: {}", id));
        self.entities.retain(|e| e.borrow().id != id)
    }

    pub fn check(&self, subject: Rc<RefCell<Entity>>, event_buffer: Rc<RefCell<EventBuffer>>) {
        let subject_box = subject.borrow().collision_box();

        for entity in self.entities.iter() {
            if entity.borrow().id == subject.borrow().id {
                continue;
            }
            let entity_box = entity.borrow().collision_box();

            if subject_box.overlaps(entity_box) {
                collides(subject.clone(), entity.clone(), event_buffer.clone());
            }
        }
    }
}
