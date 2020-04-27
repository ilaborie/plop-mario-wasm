use crate::assets::config::PlayerDefault;
use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::traits::go::Go;
use crate::entity::traits::jump::Jump;
use crate::entity::traits::EntityTrait;
use crate::entity::{DrawableEntity, Entity, EntityDisplay, ObstructionSide};
use crate::physics::jumping::Jumping;
use crate::physics::motion::{Direction, Motion};
use crate::physics::position::Position;
use crate::physics::rectangle::BoundingBox;
use crate::physics::size::Size;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PlayerEntity {
    // entity: Rc<RefCell<Entity>>,
    entity: Entity,
    jumping: Rc<RefCell<Jumping>>,
    motion: Rc<RefCell<Motion>>,
    go: Go,
    jump: Jump,
}

impl PlayerEntity {
    pub fn new(position: Position, param: &PlayerDefault) -> Self {
        let size = param.size;
        let bounding_box = BoundingBox::new(0., 0., size);
        let mut entity = Entity::new(String::from("Player"), bounding_box, size);
        entity.x = position.x();
        entity.y = position.y();

        // Traits
        let motion = Motion::new(param.motion);
        let motion = Rc::new(RefCell::new(motion));
        let go = Go::new(motion.clone());

        let jumping = Jumping::new(param.jumping);
        let jumping = Rc::new(RefCell::new(jumping));
        let jump = Jump::new(jumping.clone());

        Self {
            entity,
            jumping,
            motion,
            go,
            jump,
        }
    }

    fn is_jumping(&self) -> bool {
        self.jumping.borrow().is_jumping()
    }
    pub fn jump_start(&mut self) {
        self.jumping.borrow_mut().start();
    }
    pub fn jump_cancel(&mut self) {
        self.jumping.borrow_mut().cancel();
    }
    pub fn start_move(&mut self, direction: Direction) {
        let is_jumping = self.is_jumping();
        self.motion.borrow_mut().move_to(direction, is_jumping);
    }
    pub fn stop_move(&mut self, direction: Direction) {
        let is_jumping = self.is_jumping();
        self.motion.borrow_mut().stop(direction, is_jumping);
    }
    pub fn start_run(&mut self) {
        self.motion.borrow_mut().start_run();
    }
    pub fn stop_run(&mut self) {
        self.motion.borrow_mut().stop_run();
    }
}

impl DrawableEntity for PlayerEntity {
    fn id(&self) -> &str {
        self.entity.id.as_str()
    }
    fn entity_display(&self) -> EntityDisplay {
        if self.is_jumping() {
            return EntityDisplay::sprite(Sprite::Jump);
        }
        let distance = self.motion.borrow().distance;
        if distance > 0. {
            let dx = self.entity.dx;
            let direction = self.motion.borrow().direction;
            if (dx > 0. && direction == Direction::Left)
                || (dx < 0. && direction == Direction::Right)
            {
                EntityDisplay::sprite(Sprite::Break)
            } else {
                EntityDisplay::animation(AnimationName::Run, distance, direction)
            }
        } else {
            EntityDisplay::sprite(Sprite::Idle)
        }
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
        self.go.update(&mut self.entity, dt);
        self.jump.update(&mut self.entity, dt);
        self.entity.update(dt); // lifetime
    }

    fn obstruct(&mut self, side: ObstructionSide, rect: &BoundingBox) {
        self.go.obstruct(&mut self.entity, side, rect);
        self.jump.obstruct(&mut self.entity, side, rect);
    }
}
