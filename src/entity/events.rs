use crate::entity::Entity;
use core::cell::RefCell;
use std::any::Any;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Event {
    Stomper { id: String, y: f64 },
    Stomped(String),
    Killer(String),
    Killed(String),
    TimeOk,
    Hurry,
    Timeout,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id() && self.id() == other.id()
    }
}

impl Event {
    fn id(&self) -> Option<&str> {
        match self {
            Event::Stomper { id, .. } => Some(id),
            Event::Stomped(id) => Some(id),
            Event::Killer(id) => Some(id),
            Event::Killed(id) => Some(id),
            Event::Hurry => None,
            Event::TimeOk => None,
            Event::Timeout => None,
        }
    }
}

type Callback = Box<dyn Fn(&Event) -> ()>;

// Buffer
#[derive(Default)]
pub struct EventBuffer {
    events: Vec<Event>,
}

impl EventBuffer {
    pub fn clear(&mut self) {
        self.events.clear();
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    // Time
    pub fn time_ok(&mut self) {
        self.push_event(Event::TimeOk);
    }
    pub fn hurry(&mut self) {
        self.push_event(Event::Hurry);
    }
    pub fn timeout(&mut self) {
        self.push_event(Event::Timeout);
    }

    // Entity
    pub fn kill(&mut self, killer_id: String, killed_id: String) {
        self.push_event(Event::Killer(killer_id));
        self.push_event(Event::Killed(killed_id));
    }
    pub fn stomp(
        &mut self,
        stomper_entity: Rc<RefCell<Entity>>,
        stomped_entity: Rc<RefCell<Entity>>,
    ) {
        let top = stomped_entity.borrow().collision_box().top();
        let height = stomper_entity.borrow().size.height as f64;
        let id = stomper_entity.borrow().id.clone();
        let y = top - height;

        self.push_event(Event::Stomper { id, y });
        self.push_event(Event::Stomped(stomped_entity.borrow().id.clone()));
    }

    pub fn process_entity(&self, id: &str, callback: Callback) {
        for event in self.events.iter() {
            if event.id() == Some(id) {
                callback(event)
            }
        }
    }

    pub fn process_level(&self, callback: &dyn Fn(&Event) -> ()) {
        for event in self.events.iter() {
            if event.id().is_none() {
                callback(event)
            }
        }
    }
}
