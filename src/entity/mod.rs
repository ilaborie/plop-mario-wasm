use crate::entity::traits::EntityTrait;

pub mod player;
pub mod traits;


#[derive(Default)]
pub struct Entity {
    traits: Vec<EntityTrait>,
}

impl Entity {
    pub(crate) fn update(&mut self, dt: f64) {
        for t in self.traits.iter_mut() {
            t.update(dt);
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Velocity {
    dx: f64,
    dy: f64,
}

impl Velocity {
    pub(crate) fn new() -> Self {
        let dx = 0.0;
        let dy = 0.0;
        Self { dx, dy }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub(crate) fn new() -> Self {
        let x = 0.0;
        let y = 0.0;
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Jumping {
    duration: f64,
    velocity: f64,
    engage_time: f64,
}

impl Jumping {
    pub(crate) fn new() -> Self {
        let duration = 0.5;
        let velocity = 200.0;
        let engage_time = 0.0;
        Self { duration, velocity, engage_time }
    }
    pub(crate) fn start(&mut self) {
        self.engage_time = self.duration;
    }
    pub(crate) fn cancel(&mut self) {
        self.engage_time = 0.0;
    }
}
