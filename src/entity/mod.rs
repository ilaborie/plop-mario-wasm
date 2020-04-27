use crate::assets::sprites::{AnimationName, Sprite, SpriteSheet};
use crate::physics::motion::Direction;
use crate::physics::rectangle::BoundingBox;
use crate::physics::size::Size;
use web_sys::CanvasRenderingContext2d;

pub mod mobs;
pub mod player;
pub mod traits;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObstructionSide {
    Top,
    Right,
    Bottom,
    Left,
}

pub trait DrawableEntity {
    fn id(&self) -> &str;
    fn entity_display(&self) -> EntityDisplay;

    fn position(&self) -> (f64, f64);
    fn size(&self) -> Size;
    fn collision_box(&self) -> BoundingBox;
    fn dx(&self) -> f64;
    fn dy(&self) -> f64;

    fn apply_velocity_x(&mut self, dt: f64);
    fn apply_velocity_y(&mut self, dt: f64);
    fn apply_gravity(&mut self, dt: f64);

    fn update(&mut self, dt: f64);
    fn obstruct(&mut self, _side: ObstructionSide, rect: &BoundingBox);
}

#[derive(Debug)]
pub enum EntityDisplay {
    DisplayAnimation {
        name: AnimationName,
        distance: f64,
        direction: Direction,
    },
    DisplaySprite {
        sprite: Sprite,
    },
}

impl EntityDisplay {
    fn animation(name: AnimationName, distance: f64, direction: Direction) -> EntityDisplay {
        EntityDisplay::DisplayAnimation {
            name,
            distance,
            direction,
        }
    }

    fn sprite(sprite: Sprite) -> EntityDisplay {
        EntityDisplay::DisplaySprite { sprite }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, x: f64, y: f64, sprites: &SpriteSheet) {
        match self {
            EntityDisplay::DisplayAnimation {
                name,
                distance,
                direction,
            } => sprites.draw_tile_animation(context, *name, x, y, *distance, *direction),
            EntityDisplay::DisplaySprite { sprite } => sprites.draw_tile(context, *sprite, x, y),
        }
    }
}

pub struct Entity {
    #[allow(dead_code)]
    id: String,

    // Lifetimes
    lifetime: f64,

    // Position
    x: f64,
    y: f64,

    // Velocity
    dx: f64,
    dy: f64,

    // Bounds & Size
    bounding_box: BoundingBox,
    size: Size,
}

impl Entity {
    pub fn new(id: String, bounding_box: BoundingBox, size: Size) -> Self {
        let lifetime = 0.;
        let x = 0.;
        let y = 0.;
        let dx = 0.;
        let dy = 0.;

        Entity {
            id,
            lifetime,
            x,
            y,
            dx,
            dy,
            bounding_box,
            size,
        }
    }

    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    fn size(&self) -> Size {
        self.size
    }

    fn collision_box(&self) -> BoundingBox {
        self.bounding_box.translate(self.x, self.y)
    }

    // X, dX
    fn next_x(&self, dt: f64) -> f64 {
        self.x + self.dx * dt
    }
    pub fn apply_velocity_x(&mut self, dt: f64) {
        self.x = self.next_x(dt);
    }
    fn set_x(&mut self, x: f64, dx: f64) {
        self.x = x;
        self.dx = dx;
    }

    // Y, dY
    fn next_y(&self, dt: f64) -> f64 {
        self.y + self.dy * dt
    }
    pub fn apply_velocity_y(&mut self, dt: f64) {
        self.y = self.next_y(dt);
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
}
