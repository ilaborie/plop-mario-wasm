#[derive(Clone, Copy, Default)]
pub struct Velocity {
    dx: f64,
    dy: f64,
}

impl Velocity {
    pub fn new() -> Self {
        let dx = 0.0;
        let dy = 0.0;
        Self { dx, dy }
    }

    pub fn dx(&self) -> f64 {
        self.dx
    }
    pub fn set_dx(&mut self, dx: f64) {
        self.dx = dx;
    }

    pub fn dy(&self) -> f64 {
        self.dy
    }
    pub fn set_dy(&mut self, dy: f64) {
        self.dy = dy;
    }
    pub fn incr_dy(&mut self, dy: f64) {
        self.dy = dy;
    }
}
