use crate::assets::config::MobsDefault;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::goomba::GoombaEntity;
use crate::entity::koopa::KoopaEntity;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::EntityTrait;
use crate::physics::bounding_box::BBox;
use crate::physics::{Position, Size};
use crate::utils::log;
use core::fmt;
use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::__rt::core::fmt::Formatter;

pub mod entity_display;
pub mod entity_drawable;
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
) -> Rc<RefCell<dyn DrawableEntity>> {
    let bounding_box = param
        .bbox
        .map(|r| BBox::new(r.x as f64, r.y as f64, r.size()))
        .unwrap_or_else(|| BBox::new(0., 0., param.size));

    let mut entity = Entity::new(id, bounding_box, param.size);
    entity.dx = param.speed;
    entity.x = position.x();
    entity.y = position.y();

    match mobs {
        "goomba" => Rc::new(RefCell::new(GoombaEntity::new(entity, physics))),
        "koopa" => Rc::new(RefCell::new(KoopaEntity::new(entity, physics))),
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

pub struct Entity {
    pub(crate) id: String,
    score: Rc<Cell<u32>>,
    traits: Vec<Rc<RefCell<dyn EntityTrait>>>,

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
}

impl Entity {
    pub fn new(id: String, bounding_box: BBox, size: Size) -> Self {
        let traits = vec![];
        let lifetime = 0.;
        let living = Living::Alive;
        let x = 0.;
        let y = 0.;
        let dx = 0.;
        let dy = 0.;
        let features = vec![];
        let queue = vec![];
        let score = Rc::new(Cell::new(0));

        Entity {
            id,
            score,
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
        }
    }

    pub fn score(&self) -> Rc<Cell<u32>> {
        self.score.clone()
    }
    pub fn incr_score(&mut self, points: u32) {
        let score = self.score.get() + points;
        self.score.set(score);
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
    fn kill(&mut self, killer: &str) {
        log(&format!("{} killed by {}", self.id, killer));
        self.queue.push(Box::new(|mut e| {
            e.living = Living::Dead;
            e.dx = 0.;
            e.dy = 0.;
        }))
    }
    fn remove(&mut self) {
        log(&format!("Remove {}", self.id));
        self.living = Living::NoExistence;
    }
    fn revive(&mut self) {
        log(&format!("ReSpawn {}", self.id));
        self.living = Living::Alive;
    }

    // Features
    fn is(&self, feature: EntityFeature) -> bool {
        self.features.contains(&feature)
    }
    fn is_stomper(&self) -> bool {
        self.is(EntityFeature::Stomper)
    }
    fn is_killable(&self) -> bool {
        self.is(EntityFeature::Killable)
    }

    // Tasks
    pub fn finalize(&mut self) {
        while !self.queue.is_empty() {
            let mut task = self.queue.remove(0);
            task(self);
        }
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] traits: {:?}", self.id, self.traits)
    }
}
