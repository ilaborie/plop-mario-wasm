use crate::assets::config::MobsDefault;
use crate::audio::sounds::{AudioBoard, Fx};
use crate::entity::bullet::BulletEntity;
use crate::entity::cannon::CannonEntity;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::events::{Event, EventBuffer};
use crate::entity::goomba::GoombaEntity;
use crate::entity::koopa::KoopaEntity;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::EntityTrait;
use crate::physics::bounding_box::BBox;
use crate::physics::{Position, Size};
use crate::utils::log;
use core::fmt;
use core::fmt::Formatter;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use std::vec::Drain;
use web_sys::AudioContext;

pub mod bullet;
pub mod cannon;
pub mod entity_display;
pub mod entity_drawable;
pub mod events;
pub mod goomba;
pub mod koopa;
pub mod player;
pub mod player_env;
pub mod traits;

pub fn create_mobs(
    id: String,
    mobs: &str,
    param: &MobsDefault,
    position: Position,
    physics: Physics,
    event_buffer: Rc<RefCell<EventBuffer>>,
    audio: Option<Rc<AudioBoard>>,
) -> Rc<RefCell<dyn DrawableEntity>> {
    let bounding_box = param
        .bbox
        .map(|r| BBox::new(r.x as f64, r.y as f64, r.size()))
        .unwrap_or_else(|| BBox::new(0., 0., param.size));

    let mut entity = Entity::new(id, bounding_box, param.size, event_buffer, audio);
    entity.dx = param.speed;
    entity.x = position.x();
    entity.y = position.y();

    match mobs {
        "goomba" => Rc::new(RefCell::new(GoombaEntity::new(entity, physics))),
        "koopa" => Rc::new(RefCell::new(KoopaEntity::new(entity, physics))),
        "bullet" => Rc::new(RefCell::new(BulletEntity::new(entity))),
        "cannon" => Rc::new(RefCell::new(CannonEntity::new(entity))),
        _ => panic!("Mobs {} not found!", mobs),
    }
}

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObstructionSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EntityFeature {
    Stomper,
    Killable,
    Player,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Living {
    Alive,
    Dead,
    NoExistence,
}

type Task = Box<dyn FnMut(&mut Entity) -> ()>;

pub type EntityToCreate = (String, Rc<RefCell<dyn DrawableEntity>>);

pub struct Entity {
    pub(crate) id: String,
    traits: Vec<Rc<RefCell<dyn EntityTrait>>>,
    event_buffer: Rc<RefCell<EventBuffer>>,

    // Lifetimes
    lifetime: f64,
    living: Living,
    queue: Vec<Task>,

    // Position
    x: f64,
    y: f64,

    // Velocity
    dx: f64,
    dy: f64,

    // Bounds & Size
    bounding_box: BBox,
    size: Size,

    // Features
    features: Vec<EntityFeature>,

    // Audio
    audio_board: Option<Rc<AudioBoard>>,
    sounds: HashSet<Fx>,

    // Creation
    creation: Vec<EntityToCreate>,
}

impl Entity {
    pub fn new(
        id: String,
        bounding_box: BBox,
        size: Size,
        event_buffer: Rc<RefCell<EventBuffer>>,
        audio_board: Option<Rc<AudioBoard>>,
    ) -> Self {
        let traits = vec![];
        let lifetime = 0.;
        let living = Living::Alive;
        let x = 0.;
        let y = 0.;
        let dx = 0.;
        let dy = 0.;
        let features = vec![];
        let queue = vec![];
        let creation = vec![];
        let sounds = HashSet::new();

        Entity {
            id,
            event_buffer,
            traits,
            lifetime,
            living,
            x,
            y,
            dx,
            dy,
            bounding_box,
            size,
            features,
            queue,
            audio_board,
            sounds,
            creation,
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    // Traits
    fn add_trait(&mut self, t: Rc<RefCell<dyn EntityTrait>>) {
        self.traits.push(t.clone());
    }

    // Lifetime
    pub fn lifetime(&self) -> f64 {
        self.lifetime
    }

    pub(crate) fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub(crate) fn size(&self) -> Size {
        self.size
    }

    pub fn collision_box(&self) -> BBox {
        self.bounding_box.translate(self.x, self.y)
    }

    // X, dX
    pub fn apply_velocity_x(&mut self, dt: f64) {
        self.x += self.dx * dt
    }
    pub fn dx(&self) -> f64 {
        self.dx
    }
    fn set_x(&mut self, x: f64, dx: f64) {
        self.x = x;
        self.dx = dx;
    }

    // Y, dY
    pub fn apply_velocity_y(&mut self, dt: f64) {
        self.y += self.dy * dt
    }
    pub fn dy(&self) -> f64 {
        self.dy
    }
    fn set_y(&mut self, y: f64, dy: f64) {
        self.y = y;
        self.dy = dy;
    }

    // Gravity
    pub fn apply_gravity(&mut self, dy: f64) {
        self.dy += dy;
    }

    // Living
    pub fn living(&self) -> Living {
        self.living
    }
    fn remove(&mut self) {
        log(&format!("Remove {}", self.id));
        self.living = Living::NoExistence;
    }
    fn revive(&mut self) {
        log(&format!("Respawn {}", self.id));
        self.living = Living::Alive;
    }

    // Features
    fn is(&self, feature: EntityFeature) -> bool {
        self.features.contains(&feature)
    }
    fn is_stomper(&self) -> bool {
        self.is(EntityFeature::Stomper) && self.living == Living::Alive
    }
    fn is_killable(&self) -> bool {
        self.is(EntityFeature::Killable) && self.living == Living::Alive
    }

    // Tasks
    pub fn finalize(&mut self) {
        while !self.queue.is_empty() {
            let mut task = self.queue.remove(0);
            task(self);
        }
    }

    pub fn get_creation(&mut self) -> Drain<'_, EntityToCreate> {
        self.creation.drain(..)
    }

    // Audio
    fn play_fx(&mut self, fx: Fx) {
        self.sounds.insert(fx);
    }

    pub fn play_sounds(&mut self, audio_context: &AudioContext) {
        if let Some(ab) = &self.audio_board {
            for &fx in self.sounds.iter() {
                ab.play(audio_context, fx);
            }
        }
        self.sounds.clear();
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn finalize(event_buffer: Rc<RefCell<EventBuffer>>, entity: Rc<RefCell<Entity>>) {
    entity.borrow_mut().finalize();

    let id = entity.borrow().id.clone();

    // Events
    let e = entity.clone();
    let traits = entity.borrow().traits.clone();
    event_buffer.borrow().process_entity(
        id.as_str(),
        Box::new(move |event| {
            // log(&format!("Handle {:?}", event));
            for t in traits.iter() {
                // log(&format!("<{:?}> on {:?}", event, t.borrow().name()));
                if let Ok(mut t) = t.try_borrow_mut() {
                    match event {
                        Event::Stomper { .. } => t.on_stomper(e.clone()),
                        Event::Stomped(_) => t.on_stomped(e.clone()),
                        Event::Killer(_) => t.on_killer(e.clone()),
                        Event::Killed(_) => t.on_killed(e.clone()),
                        _ => log(&format!("Event skipped: {:?}", event)),
                    }
                } else {
                    log(&format!(
                        "Cannot process events for {} {:?}",
                        t.borrow().name(),
                        t
                    ));
                }
            }
        }),
    );
}
