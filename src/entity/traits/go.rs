use crate::assets::audio::sounds::Fx;
use crate::assets::config::MotionDefault;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living, ObstructionSide};
use crate::game::GameContext;
use crate::physics::bounding_box::BBox;
use crate::physics::Direction;
use crate::scene::level::Level;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Go {
    acceleration_base: f64,
    drag_factor_base: f64,
    deceleration_base: f64,
    count: i32,
    direction: Direction,
    heading: Direction,
    distance: f64,

    acceleration: f64,
    drag_factor: f64,
}

impl Go {
    pub fn new(motion_default: MotionDefault) -> Self {
        let acceleration_base = motion_default.acceleration_base;
        let deceleration_base = motion_default.deceleration_base;
        let direction = motion_default.direction;
        let drag_factor = motion_default.drag_factor;
        let drag_factor_base = motion_default.drag_factor;

        let count = 0;
        let heading = motion_default.direction;
        let distance = 0.;
        let acceleration = 0.;

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

    pub fn reset(&mut self) {
        self.count = 0;
        self.distance = 0.;
        self.acceleration = 0.;
    }

    pub fn direction(&self) -> Direction {
        self.heading
    }
    pub fn distance(&self) -> f64 {
        self.distance
    }

    fn direction_incr(&mut self, increment: i32, jumping: bool) {
        self.count += increment;

        self.direction = match self.count {
            x if x > 0 => {
                self.acceleration = self.acceleration_base;
                if !jumping {
                    self.heading = Direction::Right;
                }
                Direction::Right
            }
            x if x < 0 => {
                self.acceleration = self.acceleration_base;
                if !jumping {
                    self.heading = Direction::Left;
                }
                Direction::Left
            }
            _ => {
                self.acceleration = 0.0;
                Direction::Stop
            }
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

impl EntityTrait for Go {
    fn name(&self) -> &str {
        "go"
    }

    fn on_stomper(&mut self, entity: Rc<RefCell<Entity>>) {
        entity.borrow_mut().play_fx(Fx::Stomp);
        entity.borrow_mut().dy -= 500.;
    }

    fn on_killed(&mut self, entity: Rc<RefCell<Entity>>) {
        entity.borrow_mut().dx = 100.;
        entity.borrow_mut().dy = -200.;
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext, _level: &Level) {
        if entity.borrow().living != Living::Alive {
            self.count = 0;
            self.heading = Direction::Right;
            self.distance = 0.;
            self.acceleration = 0.;
            return;
        }

        let direction = self.direction;
        let factor = match direction {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
            Direction::Stop => 0.0,
        };
        let dx = entity.borrow().dx;
        let abs_dx = dx.abs();

        if direction != Direction::Stop {
            let ddx = factor * self.acceleration * context.dt();
            entity.borrow_mut().dx += ddx;
        } else if dx != 0. {
            let ddx = abs_dx.min(self.deceleration_base);
            let factor = if dx > 0. { -1. } else { 1. };
            entity.borrow_mut().dx += factor * ddx;
        } else {
            self.distance = 0.;
        }

        // Drag
        let dx = entity.borrow().dx;
        let drag = self.drag_factor / 1000. * dx * abs_dx;
        entity.borrow_mut().dx -= drag;

        self.distance += context.dt() * abs_dx;
    }

    fn obstruct(&mut self, entity: Rc<RefCell<Entity>>, side: ObstructionSide, _rect: BBox) {
        if entity.borrow().living != Living::Alive {
            return;
        }
        if let ObstructionSide::Bottom = side {
            self.heading = self.direction;
        }
    }
}
