#[derive(Deserialize, Clone, Copy, Default, Debug)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
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
