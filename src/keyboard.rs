use crate::assets::config::Configuration;
use crate::entity::animation::AnimationEntity;
use crate::physics::motion::Direction::{Left, Right};
use crate::utils::window;
use std::cell::RefCell;
use std::collections::HashMap;
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

#[derive(Default)]
pub struct Keyboard {
    key_states: Rc<RefCell<HashMap<Action, KeyState>>>,
}

impl Keyboard {
    pub fn register(&mut self, config: Configuration, player: Rc<RefCell<AnimationEntity>>) {
        let key_states = self.key_states.clone();
        let player = player.clone();
        let config = config.clone();

        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = config.action(event.code());
            if key.is_none() {
                return;
            }
            let key = key.unwrap();
            let state = KeyState::from_event_type(event.type_());
            // log(&format!("Key {:?} {:?}", key, state).to_string());

            let old = key_states.borrow_mut().insert(key, state);
            if old != Some(state) {
                match (key, state) {
                    // Jumps
                    (Action::Jump, KeyState::Pressed) => player.borrow_mut().jump_start(),
                    (Action::Jump, KeyState::Released) => player.borrow_mut().jump_cancel(),
                    // Move
                    (Action::MoveRight, KeyState::Pressed) => player.borrow_mut().start_move(Right),
                    (Action::MoveRight, KeyState::Released) => player.borrow_mut().stop_move(Right),
                    (Action::MoveLeft, KeyState::Pressed) => player.borrow_mut().start_move(Left),
                    (Action::MoveLeft, KeyState::Released) => player.borrow_mut().stop_move(Left),
                    (Action::Run, KeyState::Pressed) => player.borrow_mut().start_run(),
                    (Action::Run, KeyState::Released) => player.borrow_mut().stop_run(),
                    // Run
                    _ => {}
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
