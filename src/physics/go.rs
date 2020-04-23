#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Stop,
    Right,
}

#[derive(Clone, Copy)]
pub struct Go {
    speed_base: f64,
    pub(crate) direction: Direction,
    pub(crate) speed: f64,
}

impl Go {
    pub fn new(direction: Direction, speed_base: f64) -> Self {
        let speed = 0.0;
        Self {
            direction,
            speed,
            speed_base,
        }
    }

    pub fn move_to(&mut self, direction: Direction) {
        self.direction = direction;
        self.speed = self.speed_base;
    }
    pub fn stop(&mut self) {
        self.speed = 0.0;
        self.direction = Direction::Stop;
    }
}
