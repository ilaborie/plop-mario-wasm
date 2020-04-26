use crate::assets::config::MotionDefault;

#[derive(Serialize, Deserialize, Hash, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Stop,
    Right,
}

#[derive(Clone, Copy)]
pub struct Motion {
    acceleration_base: f64,
    drag_factor_base: f64,
    pub(crate) deceleration_base: f64,
    count: i32,
    pub(crate) direction: Direction,
    pub(crate) heading: Direction,
    pub(crate) distance: f64,

    pub(crate) acceleration: f64,
    pub(crate) drag_factor: f64,
}

impl Motion {
    pub fn new(motion_default: MotionDefault) -> Self {
        let distance = 0.;
        let count = 0;
        let heading = motion_default.direction;
        let acceleration = 0.0;
        let direction = motion_default.direction;
        let acceleration_base = motion_default.acceleration_base;
        let deceleration_base = motion_default.deceleration_base;
        let drag_factor_base = motion_default.drag_factor;
        let drag_factor = drag_factor_base / 4.;

        Self {
            direction,
            acceleration_base,
            deceleration_base,
            drag_factor_base,
            drag_factor,
            count,
            heading,
            distance,
            acceleration,
        }
    }

    fn direction_incr(&mut self, increment: i32, jumping: bool) {
        self.count += increment;

        self.direction = if self.count > 0 {
            self.acceleration = self.acceleration_base;
            if !jumping {
                self.heading = Direction::Right;
            }
            Direction::Right
        } else if self.count < 0 {
            self.acceleration = self.acceleration_base;
            if !jumping {
                self.heading = Direction::Left;
            }
            Direction::Left
        } else {
            self.acceleration = 0.0;
            Direction::Stop
        };
    }

    pub fn move_to(&mut self, direction: Direction, jumping: bool) {
        let incr = match direction {
            Direction::Left => -1,
            Direction::Stop => 0,
            Direction::Right => 1,
        };
        self.direction_incr(incr, jumping);
    }
    pub fn stop(&mut self, direction: Direction, jumping: bool) {
        let incr = match direction {
            Direction::Left => 1,
            Direction::Stop => 0,
            Direction::Right => -1,
        };
        self.direction_incr(incr, jumping);
    }

    pub fn start_run(&mut self) {
        self.drag_factor = self.drag_factor_base / 4.;
    }
    pub fn stop_run(&mut self) {
        self.drag_factor = self.drag_factor_base;
    }
}
