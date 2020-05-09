use crate::assets::config::Configuration;
use crate::physics::Direction;
use crate::physics::Direction::{Left, Right};
use crate::utils::window;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

#[derive(Deserialize, Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    MoveRight,
    MoveLeft,
    Jump,
    Down,
    Run,
    Fire,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    pub fn from_event_type(event_type: String) -> Self {
        if event_type == "keydown" {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }
}

pub trait ActionHandler {
    fn name(&self) -> String;

    fn handle(&mut self, action: Action, state: KeyState) {
        match (action, state) {
            // Jumps
            (Action::Jump, KeyState::Pressed) => self.jump_start(),
            (Action::Jump, KeyState::Released) => self.jump_cancel(),
            // Move
            (Action::MoveRight, KeyState::Pressed) => self.start_move(Right),
            (Action::MoveRight, KeyState::Released) => self.stop_move(Right),
            (Action::MoveLeft, KeyState::Pressed) => self.start_move(Left),
            (Action::MoveLeft, KeyState::Released) => self.stop_move(Left),
            // Run
            (Action::Run, KeyState::Pressed) => self.start_run(),
            (Action::Run, KeyState::Released) => self.stop_run(),
            // Down
            (Action::Down, KeyState::Pressed) => self.down(),
            // Fire
            (Action::Fire, KeyState::Pressed) => self.fire(),
            _ => {}
        }
    }

    // Jump
    fn jump_start(&mut self) {}
    fn jump_cancel(&mut self) {}

    // Move
    fn start_move(&mut self, _direction: Direction) {}
    fn stop_move(&mut self, _direction: Direction) {}

    // Run
    fn start_run(&mut self) {}
    fn stop_run(&mut self) {}

    // Down
    fn down(&mut self) {}

    // Fire
    fn fire(&mut self) {}
}

// Keyboard

type Handlers = Rc<RefCell<Vec<Rc<RefCell<dyn ActionHandler>>>>>;

pub struct Keyboard {
    keymap: HashMap<String, Action>,
    key_states: Rc<RefCell<HashMap<Action, KeyState>>>,
    handlers: Handlers,
}

impl Keyboard {
    pub fn new(config: &Configuration) -> Self {
        let keymap = config.keymap();
        let key_states = Rc::default();
        let handlers = Rc::default();

        Self {
            keymap,
            key_states,
            handlers,
        }
    }

    pub fn register(&mut self, handler: Rc<RefCell<dyn ActionHandler>>) {
        self.handlers.borrow_mut().clear();
        self.key_states.borrow_mut().clear();
        self.handlers.borrow_mut().push(handler.clone());
    }

    #[allow(dead_code)]
    pub fn unregister(&mut self, name: &str) {
        self.handlers
            .borrow_mut()
            .retain(|handler| handler.borrow().name() != name);
    }

    pub fn listen(&mut self) {
        let key_states = self.key_states.clone();
        let keymap = self.keymap.clone();
        let handlers = self.handlers.clone();

        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if let Some(action) = keymap.get(&event.code()) {
                let state = KeyState::from_event_type(event.type_());

                let old = key_states.borrow_mut().insert(*action, state);
                if (state == KeyState::Pressed && old.is_none())
                    || (old.is_some() && old != Some(state))
                {
                    for handler in handlers.borrow().iter() {
                        handler.borrow_mut().handle(*action, state);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        for event in ["keydown", "keyup"].iter() {
            window()
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .expect("Cannot listen the event");
        }
        closure.forget();
    }
}
