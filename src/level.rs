use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::sprite::SpriteEntity;
use crate::entity::Updatable;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::camera::CameraLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::sprite::SpriteLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::gravity_force::GravityForce;
use crate::physics::size::Size;
use crate::physics::tile_collider::TileCollider;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Level {
    compositor: Compositor,
    entities: Vec<Rc<RefCell<SpriteEntity>>>,
    tile_collider: Rc<TileCollider>,
    gravity: GravityForce,
}

impl Level {
    pub async fn load(level: &str, camera: Rc<RefCell<Camera>>) -> Result<Self, JsValue> {
        let gravity = GravityForce::new(4000.0);

        let level_def = LevelDefinition::load(level).await?;
        let (tiles, bg_sprites) = level_def.build().await?;
        let entities = vec![];

        let tiles = Rc::new(RefCell::new(tiles));
        let tile_collider = Rc::new(TileCollider::new(tiles.clone()));

        let mut compositor = Compositor::default();

        let bg_layer =
            BackgroundsLayer::new(tiles.clone(), bg_sprites, tile_collider.resolver().clone());
        compositor.add_layer(Rc::new(RefCell::new(bg_layer)));

        let camera_layer = CameraLayer::new(camera);
        compositor.add_layer(Rc::new(RefCell::new(camera_layer)));

        let result = Self {
            compositor,
            entities,
            tile_collider,
            gravity,
        };
        Ok(result)
    }

    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    pub fn add_entity(
        &mut self,
        entity: Rc<RefCell<SpriteEntity>>,
        sprites: Rc<SpriteSheet>,
        size: Size,
        show_collision: bool,
    ) {
        self.entities.push(entity.clone());

        if show_collision {
            let collision = CollisionLayer::new(&self, entity.clone());
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }

        //
        let layer = SpriteLayer::new(entity.clone(), sprites, size);
        self.compositor.add_layer(Rc::new(RefCell::new(layer)));
    }
}

impl Drawable for Level {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        self.compositor.draw(context, camera.clone());
    }
}

impl Updatable for Level {
    fn update(&mut self, dt: f64) {
        for entity in self.entities.iter() {
            // log(&format!("Before upd> {:?}", entity.borrow()));
            entity.borrow_mut().update(dt);

            // Position Y
            // log(&format!("Before Y> {:?}", entity.borrow()));
            let (_x, y) = entity.borrow().position();
            let (_dx, dy) = entity.borrow().velocity();
            entity.borrow_mut().set_y(y + dy * dt);
            self.tile_collider.check_y(entity.clone(), dt);

            // Position X
            // log(&format!("Before X> {:?}", entity.borrow()));
            let (x, _y) = entity.borrow().position();
            let (dx, _dy) = entity.borrow().velocity();
            entity.borrow_mut().set_x(x + dx * dt);
            self.tile_collider.check_x(entity.clone());

            // Gravity
            // log(&format!("Before Gravity> {:?}", entity.borrow()));
            let (_dx, dy) = entity.borrow().velocity();
            // log(&format!("dy {}", entity.borrow().velocity().0).to_string());
            entity.borrow_mut().set_dy(dy + self.gravity.g * dt);
        }
    }
}
