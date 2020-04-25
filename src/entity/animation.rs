use crate::assets::animations::AnimationName;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Updatable};
use crate::physics::jumping::Jumping;
use crate::physics::motion::{Direction, Motion};
use crate::physics::position::Position;
use crate::physics::size::Size;
use crate::physics::velocity::Velocity;
use fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

pub struct AnimationEntity {
    entity: Entity,
    animation: AnimationName,
    size: Rc<RefCell<Size>>,
    position: Rc<RefCell<Position>>,
    velocity: Rc<RefCell<Velocity>>,
    jumping: Rc<RefCell<Jumping>>,
    motion: Rc<RefCell<Motion>>,
}

impl AnimationEntity {
    pub fn new(animation: AnimationName, size: Size, jumping: Jumping, go: Motion) -> Self {
        let mut entity = Entity::default();
        let position = Rc::new(RefCell::new(Position::new()));
        let velocity = Rc::new(RefCell::new(Velocity::new()));
        let size = Rc::new(RefCell::new(size));
        let jumping = Rc::new(RefCell::new(jumping));
        let go = Rc::new(RefCell::new(go));

        entity
            .traits
            .push(EntityTrait::go(velocity.clone(), go.clone()));

        entity
            .traits
            .push(EntityTrait::jump(velocity.clone(), jumping.clone()));

        Self {
            entity,
            animation,
            position,
            velocity,
            jumping,
            size,
            motion: go,
        }
    }

    pub fn jump_start(&mut self) {
        self.jumping.borrow_mut().start();
    }
    pub fn jump_cancel(&mut self) {
        self.jumping.borrow_mut().cancel();
    }

    pub fn start_move(&mut self, direction: Direction) {
        self.motion.borrow_mut().move_to(direction);
    }
    pub fn stop_move(&mut self) {
        self.motion.borrow_mut().stop();
    }

    pub fn set_x(&mut self, x: f64) {
        self.position.borrow_mut().set_x(x);
    }
    pub fn set_dx(&mut self, dx: f64) {
        self.velocity.borrow_mut().set_dx(dx);
    }

    pub fn set_y(&mut self, y: f64) {
        self.position.borrow_mut().set_y(y);
    }
    pub fn set_dy(&mut self, dy: f64) {
        self.velocity.borrow_mut().set_dy(dy);
    }

    pub fn position(&self) -> (f64, f64) {
        (self.position.borrow().x(), self.position.borrow().y())
    }

    pub fn velocity(&self) -> (f64, f64) {
        (self.velocity.borrow().dx(), self.velocity.borrow().dy())
    }

    pub fn direction(&self) -> Direction {
        self.motion.borrow().heading
    }
    pub fn distance(&self) -> f64 {
        self.motion.borrow().distance
    }

    pub fn width(&self) -> u32 {
        self.size.borrow().width
    }
    pub fn height(&self) -> u32 {
        self.size.borrow().height
    }
}

impl Display for AnimationEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (x, y) = self.position();

        write!(
            f,
            "Animation<{:?}> x:{x:.1} y:{y:.1}",
            self.animation,
            x = x,
            y = y
        )
    }
}

impl Debug for AnimationEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (x, y) = self.position();
        let (dx, dy) = self.velocity();

        write!(
            f,
            "Animation<{:?}> x:{x:.1}∆{dx:.1} y:{y:.1}∆{dy:.1}",
            self.animation,
            x = x,
            y = y,
            dx = dx,
            dy = dy
        )
    }
}

impl Updatable for AnimationEntity {
    fn update(&mut self, dt: f64) {
        self.entity.update(dt);
    }
}
