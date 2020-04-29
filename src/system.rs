use crate::assets::config::Configuration;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::player::PlayerEntity;
use crate::keyboard::Keyboard;
use crate::layers::Drawable;
use crate::level::Level;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct System {
    level: Level,
    camera: Camera,
    player: Rc<RefCell<PlayerEntity>>,
}

impl System {
    pub async fn create(
        config: &Configuration,
        level: &str,
        player: &str,
    ) -> Result<Self, JsValue> {
        let camera_size = config.view * TILE_SIZE;
        let camera = Camera::new(camera_size);

        let mut level = Level::load(config, level, config.gravity).await?;

        let player = level
            .create_player(&config, player, config.player.position)
            .await?;

        // Keyboard
        let mut keyboard = Keyboard::default();
        keyboard.register(config.clone(), player.clone());

        let result = Self {
            level,
            player,
            camera,
        };

        Ok(result)
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
        self.level.draw(context, &self.camera);
        //self.level.remove_entities();
    }

    pub fn update(&mut self, dt: f64) {
        self.level.update(dt);
        self.level.remove_entities();
        self.level.respwan_entities();

        // Move camera
        let (x, _y) = self.player.borrow().position();
        let shift = (TILE_SIZE * 6) as f64;
        let cam_x = (x - shift).max(0.);
        self.camera.set_x(cam_x);
    }
}
