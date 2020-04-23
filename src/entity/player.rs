use crate::assets::{Sprite, SpriteSheet};
use crate::entity::{Entity, Position, Velocity, Jumping};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::rc::Rc;
use std::cell::RefCell;
use crate::entity::traits::EntityTrait;

#[wasm_bindgen]
pub struct PlayerEntity {
    sprites: SpriteSheet,
    sprite: Sprite,
    entity: Entity,
    position: Rc<RefCell<Position>>,
    velocity: Rc<RefCell<Velocity>>,
    jumping: Rc<RefCell<Jumping>>,
}

#[wasm_bindgen]
impl PlayerEntity {
    pub fn new(sprite: Sprite, sprites: SpriteSheet, gravity: f64) -> Self {
        let mut entity = Entity::default();
        let position = Rc::new(RefCell::new(Position::new()));
        let velocity = Rc::new(RefCell::new(Velocity::new()));
        let jumping = Rc::new(RefCell::new(Jumping::new()));

        entity.traits.push(EntityTrait::new_velocity(position.clone(), velocity.clone()));
        entity.traits.push(EntityTrait::new_gravity(velocity.clone(), gravity));
        entity.traits.push(EntityTrait::new_jump(velocity.clone(), jumping.clone()));

        Self {
            entity,
            sprite,
            sprites,
            position,
            velocity,
            jumping,
        }
    }
    pub fn jump_start(&mut self) {
        self.jumping.borrow_mut().start();
    }
    pub fn jump_cancel(&mut self) {
        self.jumping.borrow_mut().cancel();
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position.borrow_mut().x = x;
        self.position.borrow_mut().y = y;
    }

    pub fn set_velocity(&mut self, dx: f64, dy: f64) {
        self.velocity.borrow_mut().dx = dx;
        self.velocity.borrow_mut().dy = dy;
    }

    pub fn update(&mut self, dt: f64) {
        self.entity.update(dt);
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.sprites.draw_image(
            &context,
            self.sprite,
            self.position.borrow().x,
            self.position.borrow().y,
        );
    }
}
