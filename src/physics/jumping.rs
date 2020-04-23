#[derive(Clone, Copy, Default)]
pub struct Jumping {
    duration: f64,
    velocity: f64,
    engage_time: f64,
}

impl Jumping {
    pub(crate) fn new(duration: f64, velocity: f64) -> Self {
        let engage_time = 0.0;
        Self {
            duration,
            velocity,
            engage_time,
        }
    }
    pub fn velocity(&self) -> f64 {
        self.velocity
    }
    pub fn engage_time(&self) -> f64 {
        self.engage_time
    }
    pub fn decr_engage_time(&mut self, dt: f64) {
        self.engage_time -= dt;
    }

    pub fn start(&mut self) {
        // log(&"Start Jump");
        self.engage_time = self.duration;
    }
    pub fn cancel(&mut self) {
        // log(&"Stop Jump");
        self.engage_time = 0.0;
    }
}
