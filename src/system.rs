use crate::entity::player::PlayerEntity;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::Compositor;
use crate::layers::player::PlayerEntityLayer;
use crate::assets::levels::Level;
use crate::assets::sprites::SpriteSheet;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, CanvasRenderingContext2d};
use crate::utils::window;
use crate::keyboard::{KeyState, Key};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct System {
    compositor: Compositor,
    player: Rc<RefCell<PlayerEntity>>,
    key_states: Rc<RefCell<HashMap<Key, KeyState>>>,
}

impl System {
    pub fn new(
        level: Level,
        sprites: SpriteSheet,
        player_entity: PlayerEntity,
    ) -> Self {
        // Backgrounds
        let bg_layer = BackgroundsLayer::new(level.backgrounds(), &sprites);

        // Player layer
        let player = Rc::new(RefCell::new(player_entity));
        let player_layer = PlayerEntityLayer::new(player.clone());

        // Compositor
        let mut compositor = Compositor::default();
        compositor.add_layer(Rc::new(move |ctx| bg_layer.draw(ctx)));
        compositor.add_layer(Rc::new(move |ctx| player_layer.draw(ctx)));

        // KeyStates
        let key_states = Rc::new(RefCell::new(HashMap::default()));

        Self {
            compositor,
            player,
            key_states,
        }
    }


    pub fn register_keyboard(&mut self) {
        let key_states = self.key_states.clone();
        let player = self.player.clone();
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = Key::from_event_key(event.code().as_str());
            if key.is_none() { return; }
            let key = key.unwrap();
            let state = KeyState::from_event_type(event.type_());
            console_log!("Key {:?} {:?}",key, state);

            let old = key_states.borrow_mut().insert(key, state);
            if old != Some(state) {
                match (key, state) {
                    (Key::Space, KeyState::Pressed) => player.borrow_mut().jump_start(),
                    (Key::Space, KeyState::Released) => player.borrow_mut().jump_cancel(),
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(_)>);

        window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("Cannot listen the event");
        window().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .expect("Cannot listen the event");

        closure.forget();
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.compositor.draw(context);
    }

    pub fn update_player(&mut self, dt: f64) {
        self.player.borrow_mut().update(dt);
    }
}
