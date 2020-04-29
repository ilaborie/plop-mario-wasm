use crate::assets::sprites::{AnimationName, Sprite, SpriteSheet};
use crate::physics::Direction;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
pub enum EntityDisplay {
    DisplayAnimation {
        name: AnimationName,
        distance: f64,
        direction: Direction,
    },
    DisplaySprite {
        name: AnimationName,
        sprite: Sprite,
        direction: Direction,
    },
}

impl EntityDisplay {
    pub(crate) fn animation(
        name: AnimationName,
        distance: f64,
        direction: Direction,
    ) -> EntityDisplay {
        EntityDisplay::DisplayAnimation {
            name,
            distance,
            direction,
        }
    }

    pub(crate) fn sprite(
        name: AnimationName,
        sprite: Sprite,
        direction: Direction,
    ) -> EntityDisplay {
        EntityDisplay::DisplaySprite {
            name,
            sprite,
            direction,
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, x: f64, y: f64, sprites: &SpriteSheet) {
        match self {
            EntityDisplay::DisplayAnimation {
                name,
                distance,
                direction,
            } => sprites.draw_tile_animation(context, *name, x, y, *distance, *direction),
            EntityDisplay::DisplaySprite {
                name,
                sprite,
                direction,
            } => sprites.draw_tile_animation_fixed(context, *name, *sprite, x, y, *direction),
        }
    }
}
