use core::cell::RefCell;
use core::fmt::{Debug, Formatter};
use std::any::Any;
use std::fmt;
use std::rc::Rc;

use crate::assets::levels::{TriggerDefinition, TriggerKind};
use crate::entity::Entity;
use crate::game::PlayerInfo;
use crate::utils::log;

#[derive(Clone, Debug)]
pub enum Event {
    // Entity
    Stomper { id: String, y: f64 },
    Stomped(String),
    Killer(String),
    Killed(String),
    Coins(String, u32),
    // Scene
    SceneComplete,
    GotoLevel { level: String, player: PlayerInfo },
    // Time
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
            // Entity
            Event::Stomper { id, .. } => Some(id),
            Event::Stomped(id) => Some(id),
            Event::Killer(id) => Some(id),
            Event::Killed(id) => Some(id),
            Event::Coins(id, _) => Some(id),
            //  Scene
            Event::SceneComplete => None,
            Event::GotoLevel { .. } => None,
            // Time
            Event::Hurry => None,
            Event::TimeOk => None,
            Event::Timeout => None,
        }
    }

    fn is_level(&self) -> bool {
        matches!(self, Event::Hurry | Event::TimeOk | Event::Timeout)
    }

    fn is_system(&self) -> bool {
        matches!(self, Event::SceneComplete | Event::GotoLevel { .. })
    }
}

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

    // Scene
    pub fn scene_complete(&mut self) {
        log("Level complete ");
        self.push_event(Event::SceneComplete);
    }
    pub fn trigger(&mut self, trigger: TriggerDefinition, player: PlayerInfo) {
        match trigger.kind() {
            TriggerKind::Goto => {
                let level = String::from(trigger.name());
                let event = Event::GotoLevel { level, player };
                self.push_event(event)
            }
        }
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
    pub fn coin(&mut self, entity_id: String, count: u32) {
        self.push_event(Event::Coins(entity_id, count));
    }

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
        let height = stomper_entity.borrow().size().height as f64;
        let id = stomper_entity.borrow().id();
        let y = top - height;

        self.push_event(Event::Stomper { id, y });
        self.push_event(Event::Stomped(stomped_entity.borrow().id()));
    }

    pub fn drain_entity(&mut self, id: &str) -> Vec<Event> {
        let mut drain = vec![];
        let mut left = vec![];
        for event in self.events.clone().iter() {
            if event.id() == Some(id) {
                drain.push(event.clone());
            } else {
                left.push(event.clone());
            }
        }
        self.events = left;
        drain
    }

    pub fn drain_level(&mut self) -> Vec<Event> {
        let mut drain = vec![];
        let mut left = vec![];
        for event in self.events.clone().iter() {
            if event.is_level() {
                drain.push(event.clone());
            } else {
                left.push(event.clone());
            }
        }
        self.events = left;
        drain
    }

    pub fn drain_system(&mut self) -> Vec<Event> {
        let mut drain = vec![];
        let mut left = vec![];
        for event in self.events.clone().iter() {
            if event.is_system() {
                drain.push(event.clone());
            } else {
                left.push(event.clone());
            }
        }
        self.events = left;
        drain
    }
}

impl Debug for EventBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.events)
    }
}
