use crate::assets::config::Configuration;
use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::SpriteSheet;
use crate::audio::player::AudioBoard;
use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::events::EventEmitter;
use crate::entity::player::PlayerEntity;
use crate::entity::player_env::PlayerEnv;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::update;
use crate::entity::{create_mobs, Entity, EntityFeature, EntityToCreate, Living};
use crate::game::GameContext;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::entity::EntityLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::entity_collider::EntityCollider;
use crate::physics::tile_collider::TileCollider;
use crate::physics::GravityForce;
use crate::physics::Position;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Formatter;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{AudioContext, CanvasRenderingContext2d};

pub struct Level {
    name: String,
    config: Configuration,
    compositor: Compositor,
    entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    respawn_entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    tile_collider: Rc<TileCollider>,
    entity_collider: EntityCollider,
    gravity: GravityForce,
    distance: Rc<Cell<f64>>,
    next_mob: u32,
    spite_sheets: HashMap<String, Rc<SpriteSheet>>,
    audio_boards: HashMap<String, Rc<AudioBoard>>,
}

impl Level {
    pub async fn load(
        config: Configuration,
        level_name: &str,
        loading_sprites: Vec<String>,
    ) -> Result<Self, JsValue> {
        let name = String::from(level_name);
        let level_def = LevelDefinition::load(level_name).await?;
        let (backgrounds_matrix, collision_matrix, bg_sprites, gravity) = level_def.build().await?;

        let gravity = GravityForce::new(gravity.unwrap_or(config.gravity));
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

        // Sprites
        let mut spite_sheets = HashMap::new();
        for sheet in loading_sprites.iter() {
            let spite_sheet = SpriteSheet::load(sheet).await?;
            spite_sheets.insert(sheet.clone(), Rc::new(spite_sheet));
        }

        // Audio
        let mut audio_boards = HashMap::new();
        for sheet in loading_sprites.iter() {
            if let Ok(audio) = AudioBoard::load(sheet).await {
                audio_boards.insert(sheet.clone(), Rc::new(audio));
            }
        }

        let mut result = Self {
            name,
            config,
            compositor,
            entities,
            respawn_entities,
            tile_collider,
            entity_collider,
            gravity,
            distance,
            next_mob,
            spite_sheets,
            audio_boards,
        };

        for entity_def in level_def.entities() {
            result.create_mobs(entity_def.name(), entity_def.position());
        }

        Ok(result)
    }

    pub fn create_player(
        &mut self,
        player_name: &str,
        position: Position,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) -> Rc<RefCell<PlayerEnv>> {
        let physics = Physics::new(self.gravity, self.tile_collider.clone());

        let audio = self.audio_boards.get(player_name).cloned();

        let player_entity = PlayerEntity::new(
            String::from(player_name),
            position,
            &self.config.player,
            physics,
            audio,
        );
        let player = Rc::new(RefCell::new(player_entity));
        self.add_entity(player_name, player.clone());

        // Controller
        let player_env = PlayerEnv::new(player, event_emitter);
        let env = Rc::new(RefCell::new(player_env));
        self.entities.push(env.clone());

        env
    }

    pub fn create_mobs(&mut self, mob: &str, position: Position) -> Rc<RefCell<Entity>> {
        let audio = self.audio_boards.get(mob).cloned();

        let mobs_default = self
            .config
            .mobs
            .get(mob)
            .unwrap_or_else(|| panic!("No mobs configuration found for {}", mob));
        self.next_mob += 1;
        let id = format!("{} #{}", mob, self.next_mob);
        let physics = Physics::new(self.gravity, self.tile_collider.clone());

        let entity = create_mobs(id, mob, mobs_default, position, physics, audio);
        let result = entity.borrow().entity();
        self.add_entity(mob, entity.clone());
        result
    }

    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    fn add_entity(&mut self, sheet: &str, entity: Rc<RefCell<dyn DrawableEntity>>) {
        let sprites = self
            .spite_sheets
            .get(sheet)
            .unwrap_or_else(|| panic!("SpriteSheet {} not loaded", sheet));

        self.entities.push(entity.clone());
        self.entity_collider.add_entity(entity.borrow().entity());

        if self.config.dev.show_collision {
            let collision = CollisionLayer::new(&self, entity.clone());
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }

        // Entity
        let layer = EntityLayer::new(entity, sprites.clone());
        self.compositor.add_layer(Rc::new(RefCell::new(layer)));
    }

    fn remove_entities(&mut self) {
        // Fill Respawn
        for entity in self.entities.iter() {
            let removed = entity.borrow().living() == Living::NoExistence;
            let respawnable = entity.borrow().is(EntityFeature::Player);
            if removed && respawnable {
                self.respawn_entities.push(entity.clone());
            } else {
            }
        }
        // Remove
        self.entities.retain(|entity| {
            let retain = entity.borrow().living() != Living::NoExistence;
            if !retain {}
            retain
        });
    }

    fn respwan_entities(&mut self) {
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

    pub fn update(&mut self, context: &GameContext) {
        // log(&format!("Entities: {:?}", self.entities));

        self.entities_updates(context);
        self.entities_collision(context.emitter());
        self.entities_sounds(context.audio_context());
        self.entities_tasks();
        self.remove_entities();
        self.respwan_entities();

        for (key, created) in self.entities_creation() {
            self.add_entity(key.as_str(), created.clone());
        }

        let dist = self.distance.get() + 1000. * context.dt();
        self.distance.set(dist);
    }

    fn entities_updates(&mut self, context: &GameContext) {
        for entity in self.entities.iter() {
            update(entity.borrow().entity(), context);
        }
    }

    fn entities_collision(&mut self, event_emitter: Rc<RefCell<EventEmitter>>) {
        for entity in self.entities.iter() {
            self.entity_collider
                .check(entity.borrow().entity(), event_emitter.clone());
        }
    }

    fn entities_tasks(&mut self) {
        for entity in self.entities.iter() {
            entity.borrow().entity().borrow_mut().finalize();
        }
    }

    fn entities_creation(&self) -> Vec<EntityToCreate> {
        let mut result = vec![];
        for entity in self.entities.iter() {
            for e in entity.borrow().entity().borrow_mut().get_creation() {
                result.push(e);
            }
        }
        result
    }

    fn entities_sounds(&mut self, audio_context: &AudioContext) {
        for entity in self.entities.iter() {
            entity
                .borrow()
                .entity()
                .borrow_mut()
                .play_sounds(audio_context);
        }
    }
}

impl Drawable for Level {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: &Camera) {
        self.compositor.draw(context, camera);
    }
}

impl Debug for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Level <{}>", self.name)
    }
}
