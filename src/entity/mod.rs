use crate::assets::config::MobsDefault;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::goomba::GoombaEntity;
use crate::entity::koopa::KoopaEntity;
use crate::physics::bounding_box::BoundingBox;
use crate::physics::{Position, Size};
use crate::utils::log;
use std::cell::RefCell;
use std::rc::Rc;

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
) -> Rc<RefCell<dyn DrawableEntity>> {
    let bounding_box = param
        .bbox
        .map(|r| BoundingBox::new(r.x as f64, r.y as f64, r.size()))
        .unwrap_or_else(|| BoundingBox::new(0., 0., param.size));

    let mut entity = Entity::new(id, bounding_box, param.size);
    entity.dx = param.speed;
    entity.x = position.x();
    entity.y = position.y();

    match mobs {
        "goomba" => Rc::new(RefCell::new(GoombaEntity::new(entity))),
        "koopa" => Rc::new(RefCell::new(KoopaEntity::new(entity))),
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

pub struct Entity {
    id: String,

    // Lifetimes
    lifetime: f64,
    living: Living,
    can_collide: bool,

    // Position
    x: f64,
    y: f64,

    // Velocity
    pub(crate) dx: f64,
    dy: f64,

    // Bounds & Size
    bounding_box: BoundingBox,
    size: Size,

    // Features
    features: Vec<EntityFeature>,
}

impl Entity {
    pub fn new(id: String, bounding_box: BoundingBox, size: Size) -> Self {
        let lifetime = 0.;
        let living = Living::Alive;
        let x = 0.;
        let y = 0.;
        let dx = 0.;
        let dy = 0.;
        let features = vec![];
        let can_collide = true;

        Entity {
            id,
            lifetime,
            living,
            can_collide,
            x,
            y,
            dx,
            dy,
            bounding_box,
            size,
            features,
        }
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

    pub fn collision_box(&self) -> BoundingBox {
        self.bounding_box.translate(self.x, self.y)
    }

    // X, dX
    pub fn apply_velocity_x(&mut self, dt: f64) {
        self.x += self.dx * dt
    }
    fn set_x(&mut self, x: f64, dx: f64) {
        self.x = x;
        self.dx = dx;
    }

    // Y, dY
    pub fn apply_velocity_y(&mut self, dt: f64) {
        self.y += self.dy * dt
    }
    fn set_y(&mut self, y: f64, dy: f64) {
        self.y = y;
        self.dy = dy;
    }

    // Gravity
    pub fn apply_gravity(&mut self, dy: f64) {
        self.dy += dy;
    }

    // Traits update
    pub fn update(&mut self, dt: f64) {
        // Update lifetime
        self.lifetime += dt;
    }

    // Features
    pub fn living(&self) -> Living {
        self.living
    }
    fn kill(&mut self, killer: &str) {
        log(&format!("{} killed by {}", self.id, killer));
        self.living = Living::Dead;
        self.can_collide = false;
        self.dx = 0.;
        self.dy = 0.;
    }
    fn remove(&mut self) {
        log(&format!("Remove {}", self.id));
        self.living = Living::NoExistence;
    }
    fn revive(&mut self) {
        log(&format!("ReSpawn {}", self.id));
        self.can_collide = true;
        self.living = Living::Alive;
    }

    fn is(&self, feature: EntityFeature) -> bool {
        self.features.contains(&feature)
    }
    fn is_stomper(&self) -> bool {
        self.is(EntityFeature::Stomper)
    }
    fn is_killable(&self) -> bool {
        self.is(EntityFeature::Killable)
    }
}
