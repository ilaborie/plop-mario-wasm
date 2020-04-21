use web_sys::CanvasRenderingContext2d;
use crate::assets::{Sprite, SpriteSheet};
use crate::compositor::Position;

pub struct SpriteLayer {
    sprite: Sprite,
    position: Position,
    sprites: Box<SpriteSheet>,
}

impl SpriteLayer {
    pub(crate) fn new(sprite: Sprite, position: Position, sprites: Box<SpriteSheet>) -> Self {
        Self { sprite, position, sprites }
    }

    pub(crate) fn update_position(&mut self, incr_x: u32, incr_y: u32) {
        self.position.update(incr_x, incr_y);
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.sprites.draw_image(&context, self.sprite, self.position.x(), self.position.y());
    }
}
