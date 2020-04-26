use crate::assets::animations::AnimationName;
use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::animation::AnimationEntity;
use crate::entity::Updatable;
use crate::layers::animation::AnimationLayer;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::camera::CameraLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::gravity_force::GravityForce;
use crate::physics::size::Size;
use crate::physics::tile_collider::TileCollider;
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Level {
    compositor: Compositor,
    entities: Vec<Rc<RefCell<AnimationEntity>>>,
    tile_collider: Rc<TileCollider>,
    gravity: GravityForce,
    distance: Rc<Cell<f64>>,
}

impl Level {
    pub async fn load(
        level: &str,
        default_gravity: f64,
        camera: Rc<RefCell<Camera>>,
    ) -> Result<Self, JsValue> {
        let level_def = LevelDefinition::load(level).await?;
        let (tiles, bg_sprites, gravity) = level_def.build().await?;
        let gravity = GravityForce::new(gravity.unwrap_or(default_gravity));
        let entities = vec![];

        let tiles = Rc::new(RefCell::new(tiles));
        let tile_collider = Rc::new(TileCollider::new(tiles.clone()));
        let distance = Rc::new(Cell::new(0.));

        let mut compositor = Compositor::default();

        let bg_layer = BackgroundsLayer::new(
            tiles.clone(),
            bg_sprites,
            tile_collider.resolver().clone(),
            distance.clone(),
        );
        compositor.add_layer(Rc::new(RefCell::new(bg_layer)));

        let camera_layer = CameraLayer::new(camera);
        compositor.add_layer(Rc::new(RefCell::new(camera_layer)));

        let result = Self {
            compositor,
            entities,
            tile_collider,
            gravity,
            distance,
        };
        Ok(result)
    }

    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    pub fn add_entity(
        &mut self,
        entity: Rc<RefCell<AnimationEntity>>,
        sprites: Rc<SpriteSheet>,
        animation: AnimationName,
        size: Size,
        show_collision: bool,
    ) {
        self.entities.push(entity.clone());

        if show_collision {
            let collision = CollisionLayer::new(&self, entity.clone());
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }

        //
        let layer = AnimationLayer::new(entity.clone(), sprites, animation, size);
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
        self.distance.set(self.distance.get() + 1000. * dt);

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
