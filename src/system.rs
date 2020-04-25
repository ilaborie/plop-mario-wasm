use crate::assets::animations::AnimationName;
use crate::assets::sprites::SpriteSheet;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::entity::animation::AnimationEntity;
use crate::entity::{Updatable, ENTITY_SIZE};
use crate::keyboard::Key::*;
use crate::keyboard::KeyState::*;
use crate::keyboard::{Key, KeyState};
use crate::layers::Drawable;
use crate::level::Level;
use crate::physics::jumping::Jumping;
use crate::physics::motion::Direction::{Left, Right};
use crate::physics::motion::{Direction, Motion};
use crate::physics::size::Size;
use crate::utils::window;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent};

pub struct System {
    level: Rc<RefCell<Level>>,
    pub(crate) player: Rc<RefCell<AnimationEntity>>,
    key_states: Rc<RefCell<HashMap<Key, KeyState>>>,
    camera: Rc<RefCell<Camera>>,
}

impl System {
    pub async fn create(level: &str, player: &str) -> Result<Self, JsValue> {
        let camera_size = Size::new(256, 244);
        let camera = Rc::new(RefCell::new(Camera::new(camera_size)));

        let mut level = Level::load(level, camera.clone()).await?;
        let player_sprites = SpriteSheet::load(player).await?;

        let anim_player = AnimationName::Mario;
        let mut player_entity = AnimationEntity::new(
            anim_player,
            Size::new(14, TILE_SIZE),
            Jumping::new(0.25, 12_000.),
            Motion::new(Direction::Right, 8_000.),
        );
        player_entity.set_x(28.);
        player_entity.set_y(0.);

        let player = Rc::new(RefCell::new(player_entity));
        let player_size = Size::new(ENTITY_SIZE, ENTITY_SIZE);
        level.add_entity(
            player.clone(),
            Rc::new(player_sprites),
            anim_player,
            player_size,
            false,
        );

        let level = Rc::new(RefCell::new(level));

        // KeyStates
        let key_states = Rc::new(RefCell::new(HashMap::default()));

        Ok(Self {
            level,
            player,
            key_states,
            camera,
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

        for event in ["keydown", "keyup"].iter() {
            window()
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .expect("Cannot listen the event");
        }
        closure.forget();
    }

    pub fn register_mouse(&self, canvas: &HtmlCanvasElement) {
        let player = self.player.clone();
        let camera = self.camera.clone();
        let last_event_cam: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let last_event_x: Rc<Cell<i32>> = Rc::new(Cell::new(0));
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let (cam_x, cam_y) = camera.borrow().position();
            match event.buttons() {
                1 => {
                    // Control player player
                    player.borrow_mut().set_dx(0.);
                    player.borrow_mut().set_dy(0.);
                    player.borrow_mut().set_x(event.offset_x() as f64 + cam_x);
                    player.borrow_mut().set_y(event.offset_y() as f64 + cam_y);
                }
                2 => {
                    // Control camera
                    if last_event_cam.get() {
                        let dx = 2 * (event.offset_x() - last_event_x.get());
                        camera.borrow_mut().set_x(cam_x - dx as f64);
                    }
                }
                _ => {}
            }
            last_event_cam.set(event.buttons() == 2 && event.type_() == "mousemove");
            last_event_x.set(event.offset_x());
        }) as Box<dyn FnMut(_)>);

        for event in ["mousedown", "mousemove"].iter() {
            canvas
                .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                .expect("Cannot listen event");
        }
        closure.forget();

        let no_menu = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("contextmenu", no_menu.as_ref().unchecked_ref())
            .expect("Cannot listen event");
        no_menu.forget();
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
        self.level.borrow_mut().draw(context, self.camera.clone());
    }
}

impl Updatable for System {
    fn update(&mut self, dt: f64) {
        self.level.borrow_mut().update(dt);
    }
}
