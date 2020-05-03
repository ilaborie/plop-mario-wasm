use crate::entity::Entity;
use crate::utils::log;
use std::rc::Rc;
use wasm_bindgen::__rt::core::cell::RefCell;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntityEvent {
    Stomp { stomper: String, stomped: String },
    Kill { killer: String, killed: String },
}

type EventListener = Box<dyn Fn(EntityEvent) -> ()>;
type IdCallback = Box<dyn Fn(&str) -> ()>;

#[derive(Default)]
pub struct EventEmitter {
    listeners: Vec<EventListener>,
}

impl EventEmitter {
    fn emit(&self, event: EntityEvent) {
        log(&format!("Event {:?}", event));
        for listener in self.listeners.iter() {
            listener(event.clone());
        }
    }

    pub fn kill(&self, killer_entity: Rc<RefCell<Entity>>, killed_entity: Rc<RefCell<Entity>>) {
        let killer = killer_entity.borrow().id.clone();
        let killed = killed_entity.borrow().id.clone();
        killed_entity.borrow_mut().kill(killer.clone().as_str());
        self.emit(EntityEvent::Kill { killer, killed });
    }

    pub fn stomp(&self, stomper_entity: Rc<RefCell<Entity>>, stomped_entity: Rc<RefCell<Entity>>) {
        let stomper = stomper_entity.borrow().id.clone();
        let stomped = stomped_entity.borrow().id.clone();
        self.emit(EntityEvent::Stomp { stomper, stomped });
    }

    fn add_listener(&mut self, listener: EventListener) {
        self.listeners.push(listener);
    }

    pub fn on_stomp(&mut self, stomper_id: String, callback: IdCallback) {
        self.add_listener(Box::new(move |event| {
            if let EntityEvent::Stomp { stomper, stomped } = event {
                if stomper == stomper_id {
                    callback(stomped.as_str());
                }
            }
        }))
    }

    pub fn on_kill(&mut self, killer_id: String, callback: IdCallback) {
        self.add_listener(Box::new(move |event| {
            if let EntityEvent::Kill { killer, killed } = event {
                if killer == killer_id {
                    callback(killed.as_str());
                }
            }
        }))
    }

    // pub fn remove_listener(&mut self, listener: &EventListener) {
    //     let pos = self.listeners.iter().position(|x| *x == *listener );
    //     if let Some(pos) = pos {
    //         self.listeners.remove(pos);
    //     }
    // }
}
