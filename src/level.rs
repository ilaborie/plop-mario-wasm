use crate::assets::config::Configuration;
use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::player::PlayerEntity;
use crate::entity::player_env::PlayerEnv;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::update;
use crate::entity::{create_mobs, EntityFeature, Living};
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::entity::EntityLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::entity_collider::EntityCollider;
use crate::physics::tile_collider::TileCollider;
use crate::physics::GravityForce;
use crate::physics::Position;
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Level {
    compositor: Compositor,
    entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    respawn_entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    tile_collider: Rc<TileCollider>,
    entity_collider: EntityCollider,
    gravity: GravityForce,
    distance: Rc<Cell<f64>>,
    next_mob: u32,
}

impl Level {
    pub async fn load(
        config: &Configuration,
        level: &str,
        default_gravity: f64,
    ) -> Result<Self, JsValue> {
        let level_def = LevelDefinition::load(level).await?;
        let (backgrounds_matrix, collision_matrix, bg_sprites, gravity) = level_def.build().await?;

        let gravity = GravityForce::new(gravity.unwrap_or(default_gravity));
        let entities = vec![];
        let respawn_entities = vec![];
        let next_mob = 0;

        let collision = Rc::new(RefCell::new(collision_matrix));
        let tile_collider = Rc::new(TileCollider::new(collision));
        let entity_collider = EntityCollider::default();
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

        let mut result = Self {
            compositor,
            entities,
            respawn_entities,
            tile_collider,
            entity_collider,
            gravity,
            distance,
            next_mob,
        };

        for entity_def in level_def.entities() {
            result
                .create_mobs(config, entity_def.name(), entity_def.position())
                .await?;
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
        let physics = Physics::new(self.gravity, self.tile_collider.clone());
        let player_entity = PlayerEntity::new(position, &config.player, physics);

        let player = Rc::new(RefCell::new(player_entity));
        self.add_entity(player.clone(), player_sprites, config.dev.show_collision);

        // Controller
        let env = PlayerEnv::new(player.borrow().entity());
        self.entities.push(Rc::new(RefCell::new(env)));

        Ok(player)
    }

    async fn create_mobs(
        &mut self,
        config: &Configuration,
        mob: &str,
        position: Position,
    ) -> Result<(), JsValue> {
        let sprites = SpriteSheet::load(mob).await?;
        let mobs_default = config
            .mobs
            .get(mob)
            .unwrap_or_else(|| panic!("No mobs configuration found for {}", mob));
        self.next_mob += 1;
        let id = format!("{} #{}", mob, self.next_mob);
        let physics = Physics::new(self.gravity, self.tile_collider.clone());
        let entity = create_mobs(id, mob, mobs_default, position, physics);
        self.add_entity(entity, sprites, config.dev.show_collision);

        Ok(())
    }

    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    fn add_entity(
        &mut self,
        entity: Rc<RefCell<dyn DrawableEntity>>,
        sprites: SpriteSheet,
        show_collision: bool,
    ) {
        self.entities.push(entity.clone());
        self.entity_collider.add_entity(entity.borrow().entity());

        if show_collision {
            let collision = CollisionLayer::new(&self, entity.clone());
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }

        // Entity
        let layer = EntityLayer::new(entity, sprites);
        self.compositor.add_layer(Rc::new(RefCell::new(layer)));
    }

    pub(crate) fn remove_entities(&mut self) {
        // Fill Respawn
        for entity in self.entities.iter() {
            let removed = entity.borrow().living() == Living::NoExistence;
            let respawnable = entity.borrow().is(EntityFeature::Player);
            if removed && respawnable {
                self.respawn_entities.push(entity.clone());
            }
        }
        // Remove
        self.entities
            .retain(|entity| entity.borrow().living() != Living::NoExistence)
    }
    pub(crate) fn respwan_entities(&mut self) {
        // Respawn
        for entity in self.respawn_entities.iter() {
            let respawn = entity.borrow().living() == Living::Alive;
            if respawn {
                self.entities.push(entity.clone());
            }
        }
        // Remove respawn
        self.respawn_entities.retain(|entity| {
            let respawn = entity.borrow().living() == Living::Alive;
            !respawn
        })
    }

    pub fn update(&mut self, dt: f64) {
        self.entities_updates(dt);
        self.entities_collision();
        self.entities_tasks();
        self.distance.set(self.distance.get() + 1000. * dt);
    }

    fn entities_updates(&mut self, dt: f64) {
        for entity in self.entities.iter() {
            update(entity.borrow().entity(), dt);
        }
    }

    fn entities_collision(&mut self) {
        for entity in self.entities.iter() {
            self.entity_collider.check(entity.borrow().entity());
        }
    }

    fn entities_tasks(&mut self) {
        for entity in self.entities.iter() {
            entity.borrow().entity().borrow_mut().finalize();
        }
    }
}

impl Drawable for Level {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: &Camera) {
        self.compositor.draw(context, camera);
    }
}
