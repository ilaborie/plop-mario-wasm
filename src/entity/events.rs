use crate::entity::Entity;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntityEvent {
    Stomper(String),
    Stomped(String),
    Killer(String),
    Killed(String),
}

impl EntityEvent {
    fn id(&self) -> &str {
        match self {
            EntityEvent::Stomper(id) => id,
            EntityEvent::Stomped(id) => id,
            EntityEvent::Killer(id) => id,
            EntityEvent::Killed(id) => id,
        }
    }
}

type Callback = Box<dyn Fn(&EntityEvent) -> ()>;

// Buffer
#[derive(Default)]
pub struct EventBuffer {
    events: Vec<EntityEvent>,
}

impl EventBuffer {
    pub fn clear(&mut self) {
        self.events.clear();
    }

    fn push_event(&mut self, event: EntityEvent) {
        if !self.events.contains(&event) {
            self.events.push(event);
        }
    }

    // Kill
    pub fn kill(&mut self, killer_entity: Rc<RefCell<Entity>>, killed_entity: Rc<RefCell<Entity>>) {
        self.push_event(EntityEvent::Killer(killer_entity.borrow().id.clone()));
        self.push_event(EntityEvent::Killed(killed_entity.borrow().id.clone()));
    }

    // Stomp
    pub fn stomp(
        &mut self,
        stomper_entity: Rc<RefCell<Entity>>,
        stomped_entity: Rc<RefCell<Entity>>,
    ) {
        self.push_event(EntityEvent::Stomper(stomper_entity.borrow().id.clone()));
        self.push_event(EntityEvent::Stomped(stomped_entity.borrow().id.clone()));
    }

    pub fn process(&self, id: &str, callback: Callback) {
        for event in self.events.iter() {
            if event.id() == id {
                callback(event)
            }
        }
    }
}
