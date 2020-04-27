use crate::assets::config::MobsDefault;
use crate::assets::sprites::AnimationName;
use crate::entity::traits::walk::Walk;
use crate::entity::traits::EntityTrait;
use crate::entity::{DrawableEntity, Entity, EntityDisplay, ObstructionSide};
use crate::physics::motion::Direction;
use crate::physics::position::Position;
use crate::physics::rectangle::BoundingBox;
use crate::physics::size::Size;

pub struct MobsEntity {
    entity: Entity,
    animation: AnimationName,
    walk: Walk,
}

impl MobsEntity {
    pub fn new(
        id: String,
        animation: AnimationName,
        param: MobsDefault,
        position: Position,
    ) -> Self {
        let size = param.size;
        let bounding_box = param
            .bbox
            .map(|r| BoundingBox::new(r.x as f64, r.y as f64, r.size()))
            .unwrap_or_else(|| BoundingBox::new(0., 0., param.size));
        let mut entity = Entity::new(id, bounding_box, size);
        entity.dx = param.speed;
        entity.x = position.x();
        entity.y = position.y();

        // Traits
        let walk = Walk {};

        Self {
            entity,
            animation,
            walk,
        }
    }
}

impl DrawableEntity for MobsEntity {
    fn id(&self) -> &str {
        self.entity.id.as_str()
    }

    fn entity_display(&self) -> EntityDisplay {
        let dist = self.entity.lifetime;
        let dx = self.entity.dx;
        let direction = if dx < 0. {
            Direction::Left
        } else {
            Direction::Right
        };
        EntityDisplay::animation(self.animation, dist, direction)
    }

    fn position(&self) -> (f64, f64) {
        self.entity.position()
    }

    fn size(&self) -> Size {
        self.entity.size()
    }

    fn collision_box(&self) -> BoundingBox {
        self.entity.collision_box()
    }
    fn dx(&self) -> f64 {
        self.entity.dx
    }
    fn dy(&self) -> f64 {
        self.entity.dy
    }
    fn apply_velocity_x(&mut self, dt: f64) {
        self.entity.apply_velocity_x(dt);
    }

    fn apply_velocity_y(&mut self, dt: f64) {
        self.entity.apply_velocity_y(dt);
    }

    fn apply_gravity(&mut self, dt: f64) {
        self.entity.apply_gravity(dt);
    }

    fn update(&mut self, dt: f64) {
        self.walk.update(&mut self.entity, dt);
        self.entity.update(dt); // lifetime
    }

    fn obstruct(&mut self, side: ObstructionSide, rect: &BoundingBox) {
        self.walk.obstruct(&mut self.entity, side, rect);
    }
}
