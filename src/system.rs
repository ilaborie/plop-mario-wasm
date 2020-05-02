use crate::assets::config::Configuration;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
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
}

impl System {
    pub async fn create(
        config: &Configuration,
        level: &str,
        player_name: &str,
    ) -> Result<Self, JsValue> {
        let camera_size = config.view * TILE_SIZE;
        let camera = Camera::new(camera_size);

        let mut level = Level::load(config, level, config.gravity).await?;

        let player = level
            .create_player(&config, player_name, config.player.position)
            .await?;

        // Keyboard
        let mut keyboard = Keyboard::default();
        keyboard.register(config.clone(), player.clone());

        let level = Rc::new(RefCell::new(level));

        // Audio
        let audio_context = AudioContext::new().unwrap();
        let audio_context = Rc::new(audio_context);

        let result = Self {
            level,
            player,
            camera,
            audio_context,
        };
        Ok(result)
    }

    pub fn player(&self) -> Rc<RefCell<PlayerEnv>> {
        self.player.clone()
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
        self.level.borrow_mut().draw(context, &self.camera);
    }

    pub fn update(&mut self, dt: f64) {
        let context = GameContext::new(self.audio_context.clone(), self.level.clone(), dt);
        self.level.borrow_mut().update(&context);
        self.level.borrow_mut().remove_entities();
        self.level.borrow_mut().respwan_entities();

        // Move camera
        let (x, _y) = self.player.borrow().position();
        let shift = (TILE_SIZE * 6) as f64;
        let cam_x = (x - shift).max(0.);
        self.camera.set_x(cam_x);
    }
}

impl Drop for System {
    fn drop(&mut self) {
       let _ = self.audio_context.close();
    }
}
