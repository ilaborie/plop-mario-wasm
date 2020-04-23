#[derive(Clone, Copy, Default)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new() -> Self {
        let x = 0.0;
        let y = 0.0;
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
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}
