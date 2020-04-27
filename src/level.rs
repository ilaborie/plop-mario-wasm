use crate::assets::config::Configuration;
use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::{SpriteSheet, AnimationName};
use crate::camera::Camera;
use crate::entity::DrawableEntity;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::camera::CameraLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::entity::EntityLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::gravity_force::GravityForce;
use crate::physics::tile_collider::TileCollider;
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::physics::position::Position;
use crate::entity::player::PlayerEntity;
use crate::entity::mobs::MobsEntity;

pub struct Level {
    compositor: Compositor,
    entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    tile_collider: Rc<TileCollider>,
    gravity: GravityForce,
    distance: Rc<Cell<f64>>,
    next_mob: u32,
}

impl Level {
    pub async fn load(
        config: &Configuration,
        level: &str,
        default_gravity: f64,
        camera: Rc<RefCell<Camera>>,
    ) -> Result<Self, JsValue> {
        let level_def = LevelDefinition::load(level).await?;
        let (backgrounds_matrix, collision_matrix, bg_sprites, gravity) = level_def.build().await?;

        let gravity = GravityForce::new(gravity.unwrap_or(default_gravity));
        let entities = vec![];
        let next_mob = 0;

        let collision = Rc::new(RefCell::new(collision_matrix));
        let tile_collider = Rc::new(TileCollider::new(collision));
        let distance = Rc::new(Cell::new(0.));

        let mut compositor = Compositor::default();

        // Background
        let bg_sprites = Rc::new(bg_sprites);

        for background_matrix in backgrounds_matrix {
            let bg_layer = BackgroundsLayer::new(
                config.view,
                background_matrix,
                bg_sprites.clone(),
                tile_collider.resolver().clone(),
                distance.clone(),
            );
            compositor.add_layer(Rc::new(RefCell::new(bg_layer)));
        }

        // Camera
        let camera_layer = CameraLayer::new(camera);
        compositor.add_layer(Rc::new(RefCell::new(camera_layer)));

        let mut result = Self {
            compositor,
            entities,
            tile_collider,
            gravity,
            distance,
            next_mob,
        };

        for entity_def in level_def.entities() {
            result.create_mobs(config, entity_def.name(), entity_def.position()).await?;
        }

        Ok(result)
    }

    pub async fn create_player(
        &mut self,
        config: &Configuration,
        player: &str,
        position: Position,
    ) -> Result<Rc<RefCell<PlayerEntity>>, JsValue> {
        let player_sprites = SpriteSheet::load(player).await?;
        let player_entity = PlayerEntity::new(position, &config.player);

        let player = Rc::new(RefCell::new(player_entity));
        self.add_entity(player.clone(), player_sprites, config.dev.show_collision);

        Ok(player)
    }

    async fn create_mobs(
        &mut self,
        config: &Configuration,
        mob: &str,
        position: Position,
    ) -> Result<(), JsValue> {
        let animation = AnimationName::Walk;
        let sprites = SpriteSheet::load(mob).await?;
        let mobs_default = config.mobs[mob];
        self.next_mob += 1;
        let id = format!("{} #{}", mob, self.next_mob);
        let entity = MobsEntity::new(id, animation, mobs_default, position);
        let entity = Rc::new(RefCell::new(entity));
        self.add_entity(entity, sprites, config.dev.show_collision);

        Ok(())
    }

    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    pub fn add_entity(
        &mut self,
        entity: Rc<RefCell<dyn DrawableEntity>>,
        sprites: SpriteSheet,
        show_collision: bool,
    ) {
        self.entities.push(entity.clone());

        if show_collision {
            let collision = CollisionLayer::new(&self, entity.clone());
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }

        // Entity
        let layer = EntityLayer::new(entity, sprites);
        self.compositor.add_layer(Rc::new(RefCell::new(layer)));
    }

    pub fn update(&mut self, dt: f64) {
        self.distance.set(self.distance.get() + 1000. * dt);

        for entity in self.entities.iter() {
            // log(&format!("Before upd> {:?}", entity.borrow()));
            entity.borrow_mut().update(dt);

            // Position Y
            // log(&format!("Before Y> {:?}", entity.borrow()));
            self.tile_collider.check_y(entity.clone());
            entity.borrow_mut().apply_velocity_x(dt);

            // Position X
            // log(&format!("Before X> {:?}", entity.borrow()));
            self.tile_collider.check_x(entity.clone());
            entity.borrow_mut().apply_velocity_y(dt);

            // Gravity
            // log(&format!("Before Gravity> {:?}", entity.borrow()));
            entity.borrow_mut().apply_gravity(self.gravity.g * dt);
        }
    }
}

impl Drawable for Level {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        self.compositor.draw(context, camera);
    }
}
