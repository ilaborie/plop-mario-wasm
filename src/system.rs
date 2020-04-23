use crate::entity::sprite::SpriteEntity;
use crate::entity::Updatable;
use crate::keyboard::Key::*;
use crate::keyboard::KeyState::*;
use crate::keyboard::{Key, KeyState};
use crate::layers::level::LevelEntity;
use crate::layers::Drawable;
use crate::physics::go::Direction::{Left, Right};
use crate::utils::window;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, MouseEvent};
use crate::assets::levels::load_level;
use crate::assets::{load_background_sprites, load_player_sprites};
use crate::assets::sprites::Sprite;
use crate::physics::size::Size;
use crate::physics::jumping::Jumping;
use crate::physics::go::{Go, Direction};

pub struct System {
    level: Rc<RefCell<LevelEntity>>,
    pub(crate) player: Rc<RefCell<SpriteEntity>>,
    key_states: Rc<RefCell<HashMap<Key, KeyState>>>,
}

impl System {
    pub async fn create(level: &str) -> Result<Self, JsValue> {
        let level = load_level(level).await?;
        let bg_sprites = load_background_sprites().await?;
        let player_sprites = load_player_sprites().await?;

        let mut level_entity = LevelEntity::new(level);
        level_entity.add_background(&bg_sprites);

        let mut player_entity = SpriteEntity::new(
            Sprite::MarioIdle,
            player_sprites,
            Size::new(14, 16),
            Jumping::new(0.25, 12_000.0),
            Go::new(Direction::Right, 8_000.0),
        );
        player_entity.set_x(28.0);
        player_entity.set_y(0.0);

        let player = Rc::new(RefCell::new(player_entity));
        level_entity.add_entity(player.clone());

        let level = Rc::new(RefCell::new(level_entity));

        // KeyStates
        let key_states = Rc::new(RefCell::new(HashMap::default()));

        Ok(Self {
            level,
            player,
            key_states,
        })
    }

    pub fn register_keyboard(&mut self) {
        let key_states = self.key_states.clone();
        let player = self.player.clone();
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = Key::from_event_key(event.code().as_str());
            if key.is_none() {
                return;
            }
            let key = key.unwrap();
            let state = KeyState::from_event_type(event.type_());
            // log(&format!("Key {:?} {:?}", key, state).to_string());

            let old = key_states.borrow_mut().insert(key, state);
            if old != Some(state) {
                match (key, state) {
                    (Space, Pressed) => player.borrow_mut().jump_start(),
                    (Space, Released) => player.borrow_mut().jump_cancel(),
                    (ArrowRight, Pressed) => player.borrow_mut().start_move(Right),
                    (ArrowRight, Released) => player.borrow_mut().stop_move(),
                    (ArrowLeft, Pressed) => player.borrow_mut().start_move(Left),
                    (ArrowLeft, Released) => player.borrow_mut().stop_move(),
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(_)>);

        for event in vec!["keydown", "keyup"] {
            window()
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .expect("Cannot listen the event");
        }
        closure.forget();
    }

    pub fn debug_collision(&self, canvas: &HtmlCanvasElement) {
        let player = self.player.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            if event.buttons() == 1 {
                player.borrow_mut().set_dx(0.0);
                player.borrow_mut().set_dy(0.0);
                player.borrow_mut().set_x(event.offset_x() as f64);
                player.borrow_mut().set_y(event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);

        for event in vec!["mousedown", "mousemove"] {
            canvas
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .expect("Cannot listen event");
        }
        closure.forget();
    }
}

impl Drawable for System {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.level.borrow().draw(context);
    }
}

impl Updatable for System {
    fn update(&mut self, dt: f64) {
        self.level.borrow_mut().update(dt);
    }
}
