use crate::assets::config::JumpingDefault;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::bounding_box::BoundingBox;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Jump {
    ready: bool,
    duration: f64,
    velocity: f64,
    speed_boost: f64,
    engage_time: f64,
    request_time: f64,
    grace_period: f64,
}

impl Jump {
    pub fn new(jumping_default: JumpingDefault) -> Self {
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

    fn engage(&mut self, dt: f64) {
        if self.ready {
            self.engage_time = self.duration;
            self.request_time = 0.;
            self.ready = false;
        }
        self.request_time -= dt;
    }

    pub(crate) fn is_jumping(&self) -> bool {
        !self.ready
    }

    pub(crate) fn start(&mut self) {
        // log(&"Start Jump");
        self.request_time = self.grace_period;
    }
    pub(crate) fn cancel(&mut self) {
        // log(&"Stop Jump");
        self.engage_time = 0.0;
        self.request_time = 0.0;
    }
}

impl EntityTrait for Jump {
    fn update(&mut self, entity: Rc<RefCell<Entity>>, dt: f64) {
        if self.request_time > 0. {
            self.engage(dt);
        }
        if self.engage_time > 0. {
            let dx = entity.borrow().dx;
            let speed_bonus = dx.abs() * self.speed_boost;
            let dy = -(self.velocity + speed_bonus) * dt;
            entity.borrow_mut().dy = dy;
            self.engage_time -= dt;
        }
    }

    fn obstruct(
        &mut self,
        _entity: Rc<RefCell<Entity>>,
        side: ObstructionSide,
        _rect: BoundingBox,
    ) {
        match side {
            ObstructionSide::Bottom => {
                self.ready = true;
            }
            ObstructionSide::Top => self.cancel(),
            _ => {}
        }
    }

    fn collides(&mut self, us: Rc<RefCell<Entity>>, them: Rc<RefCell<Entity>>) {
        let stomper = us.borrow().is_stomper();
        let killable = them.borrow().is_killable();
        if stomper && killable && us.borrow().dy > them.borrow().dy {
            self.ready = true;
        }
    }
}
