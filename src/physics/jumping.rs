use crate::assets::config::JumpingDefault;

#[derive(Clone, Copy, Default)]
pub struct Jumping {
    ready: bool,
    duration: f64,
    velocity: f64,
    speed_boost: f64,
    engage_time: f64,
    request_time: f64,
    grace_period: f64,
}

impl Jumping {
    pub(crate) fn new(jumping_default: JumpingDefault) -> Self {
        let engage_time = 0.;
        let request_time = 0.;
        let ready = false;
        let duration = jumping_default.duration;
        let velocity = jumping_default.velocity;
        let grace_period = jumping_default.grace_period;
        let speed_boost = jumping_default.speed_boost;
        Self {
            ready,
            duration,
            velocity,
            engage_time,
            request_time,
            grace_period,
            speed_boost,
        }
    }

    pub fn speed_boost(&self) -> f64 {
        self.speed_boost
    }
    pub fn velocity(&self) -> f64 {
        self.velocity
    }
    pub fn engage_time(&self) -> f64 {
        self.engage_time
    }
    pub fn request_time(&self) -> f64 {
        self.request_time
    }
    pub fn decr_engage_time(&mut self, dt: f64) {
        self.engage_time -= dt;
    }
    pub fn is_jumping(&self) -> bool {
        !self.ready
    }

    // pub fn clear_ready(&mut self) {
    //     self.ready = false;
    // }
    pub fn set_ready(&mut self) {
        self.ready = true;
    }

    pub fn start(&mut self) {
        // log(&"Start Jump");
        self.request_time = self.grace_period;
    }
    pub fn engage(&mut self, dt: f64) {
        if self.ready {
            self.engage_time = self.duration;
            self.request_time = 0.;
            self.ready = false;
        }
        self.request_time -= dt;
    }
    pub fn cancel(&mut self) {
        // log(&"Stop Jump");
        self.engage_time = 0.0;
        self.request_time = 0.0;
    }
}
