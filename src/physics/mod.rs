use std::ops::Mul;

pub mod bounding_box;
pub mod entity_collider;
pub mod matrix;
pub mod tile_collider;
pub mod tile_resolver;

#[derive(Clone, Copy, Default)]
pub struct GravityForce {
    pub(crate) g: f64,
}

impl GravityForce {
    pub(crate) fn new(g: f64) -> Self {
        Self { g }
    }
}

#[derive(Deserialize, Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Stop,
    Right,
}

#[derive(Deserialize, Clone, Copy, Default, Debug)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

impl From<(u32, u32)> for Position {
    fn from(pos: (u32, u32)) -> Self {
        let x = pos.0 as f64;
        let y = pos.1 as f64;

        Self { x, y }
    }
}

#[derive(Deserialize, Clone, Copy, Default, Debug)]
pub struct Size {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Self::Output {
        Size::new(self.width * rhs, self.height * rhs)
    }
}
