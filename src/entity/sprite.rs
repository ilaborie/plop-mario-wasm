use crate::assets::sprites::Sprite;
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

pub struct SpriteEntity {
    sprite: Sprite,
    entity: Entity,
    size: Rc<RefCell<Size>>,
    position: Rc<RefCell<Position>>,
    velocity: Rc<RefCell<Velocity>>,
    jumping: Rc<RefCell<Jumping>>,
    go: Rc<RefCell<Motion>>,
}

impl SpriteEntity {
    pub fn new(sprite: Sprite, size: Size, jumping: Jumping, go: Motion) -> Self {
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
            sprite,
            position,
            velocity,
            jumping,
            size,
            go,
        }
    }

    pub fn sprite(&self) -> Sprite {
        self.sprite
    }
    pub fn jump_start(&mut self) {
        self.jumping.borrow_mut().start();
    }
    pub fn jump_cancel(&mut self) {
        self.jumping.borrow_mut().cancel();
    }

    pub fn start_move(&mut self, direction: Direction) {
        self.go.borrow_mut().move_to(direction);
    }
    pub fn stop_move(&mut self) {
        self.go.borrow_mut().stop();
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

    pub fn width(&self) -> u32 {
        self.size.borrow().width
    }
    pub fn height(&self) -> u32 {
        self.size.borrow().height
    }
}

impl Display for SpriteEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (x, y) = self.position();

        write!(
            f,
            "Entity<{:?}> x:{x:.1} y:{y:.1}",
            self.sprite,
            x = x,
            y = y
        )
    }
}

impl Debug for SpriteEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (x, y) = self.position();
        let (dx, dy) = self.velocity();

        write!(
            f,
            "Entity<{:?}> x:{x:.1}∆{dx:.1} y:{y:.1}∆{dy:.1}",
            self.sprite,
            x = x,
            y = y,
            dx = dx,
            dy = dy
        )
    }
}

impl Updatable for SpriteEntity {
    fn update(&mut self, dt: f64) {
        self.entity.update(dt);
    }
}
