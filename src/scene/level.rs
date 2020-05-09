use crate::assets::audio::MusicController;
use crate::assets::levels::TriggerDefinition;
use crate::assets::sprites::SpriteSheet;
use crate::assets::{Assets, TILE_SIZE};
use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::player::PlayerEntity;
use crate::entity::player_env::PlayerEnv;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::update;
use crate::entity::trigger::TriggerEntity;
use crate::entity::{create_mobs, finalize, Entity, EntityFeature, EntityToCreate, Living};
use crate::events::{Event, EventBuffer};
use crate::game::{GameContext, PlayerInfo};
use crate::input::Keyboard;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::dashboard::Dashboard;
use crate::layers::entity::EntityLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::entity_collider::EntityCollider;
use crate::physics::tile_collider::TileCollider;
use crate::physics::Position;
use crate::physics::{GravityForce, Size};
use crate::scene::Scene;
use crate::utils::log;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Formatter;
use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;
use web_sys::AudioContext;

pub struct Level {
    name: String,
    assets: Assets,
    size: Size,
    camera: Camera,
    dashboard: Dashboard,
    compositor: Compositor,
    entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    respawn_entities: Vec<Rc<RefCell<dyn DrawableEntity>>>,
    tile_collider: Rc<RefCell<TileCollider>>,
    entity_collider: EntityCollider,
    player_env: Option<Rc<RefCell<PlayerEnv>>>,
    gravity: GravityForce,
    distance: Rc<Cell<f64>>,
    next_mob: u32,
    music_controller: Rc<MusicController>,
}

impl Level {
    pub fn new(level_name: &str, assets: Assets) -> Self {
        let specs = assets.level(level_name);
        let config = assets.configuration();

        let name = String::from(level_name);
        let camera_size = config.view * TILE_SIZE;
        let camera = Camera::new(camera_size);

        let dashboard = Dashboard::new(assets.font());

        let matrix = specs.tiles();
        let size = matrix.first().unwrap().borrow().size();

        let entities = vec![];
        let respawn_entities = vec![];
        let next_mob = 0;

        let tile_collider = Rc::new(RefCell::new(TileCollider::new(&matrix)));
        let entity_collider = EntityCollider::default();
        let gravity = GravityForce::new(specs.gravity().unwrap_or(config.gravity));
        let player_env = None;
        let distance = Rc::new(Cell::new(0.));

        // Compositor & layers
        let mut compositor = Compositor::default();
        let bg_sprites = specs.bg_sprites();
        for background_matrix in matrix.iter() {
            let bg_layer = BackgroundsLayer::new(
                config.view,
                background_matrix.clone(),
                bg_sprites.clone(),
                distance.clone(),
            );
            compositor.add_layer(Rc::new(RefCell::new(bg_layer)));
        }

        let music_player = assets.music_player(specs.music());
        let music_controller = MusicController::new(music_player);
        let music_controller = Rc::new(music_controller);

        let mut result = Self {
            name,
            camera,
            dashboard,
            size,
            compositor,
            entities,
            respawn_entities,
            next_mob,
            tile_collider,
            entity_collider,
            gravity,
            player_env,
            distance,
            assets,
            music_controller,
        };

        // Entities
        for entity_def in specs.entities() {
            result.create_mobs(entity_def.name(), entity_def.position());
        }

        // Triggers
        for trigger in specs.triggers().iter() {
            result.create_trigger(trigger.clone());
        }

        result
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn find_player(&self) -> Option<Rc<RefCell<PlayerEnv>>> {
        self.player_env.clone()
    }

    pub fn current_player(&self) -> PlayerInfo {
        let player = self
            .find_player()
            .unwrap_or_else(|| panic!("No player found"));
        let name = player.borrow().name();
        let lives = player.borrow().lives().get();
        let score = player.borrow().score().get();
        let coins = player.borrow().coins().get();

        PlayerInfo::new(name.as_str(), lives, score, coins)
    }

    pub fn start_or_resume(&mut self, player_info: &PlayerInfo, input: Rc<RefCell<Keyboard>>) {
        let position = Position::new(8., 64.);
        if let Some(player) = self.find_player() {
            log(&format!("Update player {:?}", player));
            player.borrow_mut().update_player(player_info, position);
            self.music_controller.play_theme();
            input.borrow_mut().register(player);
        } else {
            let player = self.create_player(&player_info, position);
            input.borrow_mut().register(player);
        }
    }

    pub fn create_player(
        &mut self,
        player_info: &PlayerInfo,
        position: Position,
    ) -> Rc<RefCell<PlayerEnv>> {
        let physics = Physics::new(self.gravity, self.tile_collider.clone());

        let audio = self.assets.audio_board(player_info.name());

        let config_player = self.assets.configuration().player;
        let player_entity =
            PlayerEntity::new(player_info, position, &config_player, physics, audio);
        let player = Rc::new(RefCell::new(player_entity));
        self.add_entity(player_info.name(), player.clone());

        // Controller
        let player_env = PlayerEnv::new(player);
        let env = Rc::new(RefCell::new(player_env));
        self.entities.push(env.clone());

        self.player_env = Some(env.clone());

        env
    }

    pub fn create_mobs(&mut self, mob: &str, position: Position) -> Rc<RefCell<Entity>> {
        let audio = self.assets.audio_board(mob);
        let config = self.assets.configuration();
        let mobs_default = config
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

    fn create_trigger(&mut self, trigger: TriggerDefinition) {
        let entity = TriggerEntity::new(trigger);
        let entity = Rc::new(RefCell::new(entity));

        self.collision_layer(entity.clone());

        self.entities.push(entity);
    }

    pub fn sprite_sheet(&self, name: &str) -> Rc<SpriteSheet> {
        self.assets.spite_sheet(name)
    }

    fn add_entity(&mut self, sheet: &str, entity: Rc<RefCell<dyn DrawableEntity>>) {
        let sprites = self.sprite_sheet(sheet);
        self.entities.push(entity.clone());
        self.entity_collider.add_entity(entity.borrow().entity());

        self.collision_layer(entity.clone());

        // Entity
        let layer = EntityLayer::new(entity, sprites);
        self.compositor.add_layer(Rc::new(RefCell::new(layer)));
    }

    fn collision_layer(&mut self, entity: Rc<RefCell<dyn DrawableEntity>>) {
        if self.assets.configuration().dev.show_collision {
            let collision = CollisionLayer::new(entity);
            self.compositor.add_layer(Rc::new(RefCell::new(collision)));
        }
    }

    fn remove_entities(&mut self) {
        // Fill Respawn
        for entity in self.entities.iter() {
            let removed = entity.borrow().living() == Living::NoExistence;
            let respawnable = entity.borrow().is(EntityFeature::Player);
            if removed && respawnable {
                self.respawn_entities.push(entity.clone());
            } else if removed {
                self.entity_collider
                    .remove_entity(entity.borrow().id().as_str());
            }
        }

        // Remove
        self.entities
            .retain(|entity| entity.borrow().living() != Living::NoExistence);
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

    fn focus_player(&mut self) {
        if let Some(player_env) = self.player_env.clone() {
            let width = self.size.width - 16;
            let max_x = (width * TILE_SIZE) as f64;

            let (x, _y) = player_env.borrow().position();
            let shift = (TILE_SIZE * 6) as f64;
            let cam_x = (x - shift).max(0.).min(max_x);
            self.camera.set_x(cam_x);
        }
    }

    fn handle_level_event(&self, event: &Event) {
        match event {
            Event::TimeOk => self.music_controller.play_theme(),
            Event::Hurry => self.music_controller.play_hurry(),
            _ => {}
        }
    }

    fn entities_updates(&self, context: &GameContext) {
        for entity in self.entities.iter() {
            update(entity.borrow().entity(), context, self);
        }
    }

    fn entities_collision(&self, event_emitter: Rc<RefCell<EventBuffer>>) {
        for entity in self.entities.iter() {
            // log(&format!("Check collision for {:?}", entity.borrow().id()));
            self.entity_collider
                .check(entity.borrow().entity(), event_emitter.clone());
        }
    }

    fn entities_tasks(&self, event_buffer: Rc<RefCell<EventBuffer>>) {
        for entity in self.entities.iter() {
            finalize(event_buffer.clone(), entity.borrow().entity());
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

    fn entities_sounds(&self, audio_context: &AudioContext) {
        for entity in self.entities.iter() {
            entity
                .borrow()
                .entity()
                .borrow_mut()
                .play_sounds(audio_context);
        }
    }
}

impl Debug for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Level <{}>", self.name)
    }
}

impl Scene for Level {
    fn update_soft(&self, context: &GameContext) {
        self.entities_updates(context);
        self.entities_collision(context.emitter());
        self.entities_sounds(context.audio_context());
        self.entities_tasks(context.emitter());

        // Dashboard
        self.dashboard.draw(&context, self);
    }

    fn update(&mut self, context: &GameContext) {
        // Entity remove / respawn / create
        self.remove_entities();
        self.respwan_entities();
        for (key, created) in self.entities_creation() {
            self.add_entity(key.as_str(), created.clone());
        }

        // Level Distance
        let dist = self.distance.get() + 1000. * context.dt();
        self.distance.set(dist);

        // Camera
        self.focus_player();

        // Level Events
        for event in context.emitter().borrow_mut().drain_level().iter() {
            self.handle_level_event(event);
        }
    }

    fn draw(&mut self, context: &GameContext) {
        self.compositor.draw(context.video_context(), &self.camera);
        self.dashboard.draw(context, self);
    }

    fn pause(&mut self) {
        // Stop music
        self.music_controller.pause();
    }
}
