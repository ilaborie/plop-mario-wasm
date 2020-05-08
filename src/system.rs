use crate::assets::config::Configuration;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::entity::events::EventBuffer;
use crate::entity::player_env::PlayerEnv;
use crate::game::GameContext;
use crate::input::Keyboard;
use crate::layers::Drawable;
use crate::level::Level;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, CanvasRenderingContext2d};

pub struct System {
    level: Rc<RefCell<Level>>,
    camera: Camera,
    player: Rc<RefCell<PlayerEnv>>,
    audio_context: Rc<AudioContext>,
    event_buffer: Rc<RefCell<EventBuffer>>,
}

impl System {
    pub async fn create(
        config: Configuration,
        level: &str,
        player_name: &str,
    ) -> Result<Self, JsValue> {
        let camera_size = config.view * TILE_SIZE;
        let camera = Camera::new(camera_size);

        // Events
        let event_buffer: Rc<RefCell<EventBuffer>> = Rc::default();

        let sprite_sheets = vec![
            String::from(player_name),
            String::from("bullet"),
            String::from("cannon"),
            String::from("goomba"),
            String::from("koopa"),
        ];
        let mut level =
            Level::load(config.clone(), level, sprite_sheets, event_buffer.clone()).await?;

        let position = config.player.position;
        let player = level.create_player(player_name, position, event_buffer.clone());

        // Keyboard
        let mut keyboard = Keyboard::default();
        keyboard.register(config.clone(), player.clone());

        // Audio
        let audio_context = AudioContext::new().unwrap();
        let audio_context = Rc::new(audio_context);

        let level = Rc::new(RefCell::new(level));
        let result = Self {
            level,
            player,
            camera,
            audio_context,
            event_buffer,
        };
        Ok(result)
    }

    pub fn player(&self) -> Rc<RefCell<PlayerEnv>> {
        self.player.clone()
    }

    pub fn level(&self) -> Rc<RefCell<Level>> {
        self.level.clone()
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
        self.level.borrow_mut().draw(context, &self.camera);
    }

    pub fn update(&mut self, dt: f64) {
        let context = GameContext::new(
            self.audio_context.clone(),
            self.event_buffer.clone(),
            self.player.clone(),
            dt,
        );

        self.level.borrow_mut().update(&context);

        self.event_buffer.borrow_mut().clear();

        // Move camera
        let width = self.level.borrow().size().width - 16;
        let max_x = (width * TILE_SIZE) as f64;

        let (x, _y) = self.player.borrow().position();
        let shift = (TILE_SIZE * 6) as f64;
        let cam_x = (x - shift).max(0.).min(max_x);
        self.camera.set_x(cam_x);
    }
}

impl Drop for System {
    fn drop(&mut self) {
        let _ = self.audio_context.close();
    }
}
