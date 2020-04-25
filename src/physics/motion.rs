#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Stop,
    Right,
}

#[derive(Clone, Copy)]
pub struct Motion {
    speed_base: f64,
    pub(crate) direction: Direction,
    pub(crate) heading: Direction,
    pub(crate) speed: f64,
    pub(crate) distance: f64,
}

impl Motion {
    pub fn new(direction: Direction, speed_base: f64) -> Self {
        let speed = 0.0;
        let distance = 0.0;
        let heading = direction;
        Self {
            direction,
            heading,
            speed,
            speed_base,
            distance,
        }
    }

    pub fn move_to(&mut self, direction: Direction) {
        self.direction = direction;
        self.heading = direction;
        self.speed = self.speed_base;
    }
    pub fn stop(&mut self) {
        self.speed = 0.0;
        self.direction = Direction::Stop;
    }
}
