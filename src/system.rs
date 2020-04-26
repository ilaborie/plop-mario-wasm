use crate::assets::animations::AnimationName;
use crate::assets::config::Configuration;
use crate::assets::sprites::SpriteSheet;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::entity::animation::AnimationEntity;
use crate::entity::{Updatable, ENTITY_SIZE};
use crate::keyboard::Keyboard;
use crate::layers::Drawable;
use crate::level::Level;
use crate::physics::jumping::Jumping;
use crate::physics::motion::Motion;
use crate::physics::size::Size;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct System {
    config: Configuration,
    level: Rc<RefCell<Level>>,
    player: Rc<RefCell<AnimationEntity>>,
    keyboard: Keyboard,
    camera: Rc<RefCell<Camera>>,
}

impl System {
    pub async fn create(config: Configuration, level: &str, player: &str) -> Result<Self, JsValue> {
        let camera_size = Size::new(256, 244); // FIXME to config
        let camera = Rc::new(RefCell::new(Camera::new(camera_size)));

        let mut level = Level::load(level, config.physics.gravity, camera.clone()).await?;
        let player_sprites = SpriteSheet::load(player).await?;

        let anim_player = AnimationName::Mario;
        let mut player_entity = AnimationEntity::new(
            anim_player,
            Size::new(14, TILE_SIZE), // FIXME maybe in animation value ?
            Jumping::new(config.physics.jumping),
            Motion::new(config.physics.motion),
        );
        // FIXME level initial position
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

        // Keyboard
        let keyboard = Keyboard::default();

        Ok(Self {
            config,
            level,
            player,
            keyboard,
            camera,
        })
    }

    pub fn register_keyboard(&mut self) {
        self.keyboard
            .register(self.config.clone(), self.player.clone());
    }

    pub fn draw(&mut self, context: &CanvasRenderingContext2d) {
        self.level.borrow_mut().draw(context, self.camera.clone());
    }
}

impl Updatable for System {
    fn update(&mut self, dt: f64) {
        self.level.borrow_mut().update(dt);

        let (x, _y) = self.player.borrow().position();
        let shift = (TILE_SIZE * 6) as f64;
        if x > shift {
            self.camera.borrow_mut().set_x(x - shift);
        }
    }
}
