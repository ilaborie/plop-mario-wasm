use crate::entity::events::EventEmitter;
use crate::entity::traits::walk::Walk;
use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, Living};
use crate::game::GameContext;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KoopaState {
    Walking,
    Hiding,
    Panic,
}

impl Default for KoopaState {
    fn default() -> Self {
        KoopaState::Walking
    }
}

pub struct KoopaBehavior {
    state: KoopaState,
    walk: Rc<RefCell<Walk>>,
    hide_time: f64,
    hide_duration: f64,
    walk_speed: f64,
    panic_speed: f64,
}

impl KoopaBehavior {
    pub fn new(walk: Rc<RefCell<Walk>>) -> Self {
        let state = KoopaState::default();
        let hide_time = 0.;
        let hide_duration = 5.;
        let walk_speed = 0.;
        let panic_speed = 300.;
        Self {
            state,
            walk,
            hide_time,
            hide_duration,
            walk_speed,
            panic_speed,
        }
    }

    pub fn state(&self) -> KoopaState {
        self.state
    }
    pub fn hide_time(&self) -> f64 {
        self.hide_time
    }

    fn hide(&mut self, us: Rc<RefCell<Entity>>) {
        us.borrow_mut().dx = 0.;
        self.walk.borrow_mut().disable();
        if self.walk_speed == 0. {
            self.walk_speed = self.walk.borrow().speed();
        }
        self.hide_time = 0.;
        self.state = KoopaState::Hiding;
    }

    fn unhide(&mut self, _us: Rc<RefCell<Entity>>) {
        self.walk.borrow_mut().enable();
        self.walk.borrow_mut().set_speed(self.walk_speed);
        self.state = KoopaState::Walking;
    }

    fn panic(&mut self, _us: Rc<RefCell<Entity>>, them: Rc<RefCell<Entity>>) {
        self.walk.borrow_mut().enable();
        self.walk
            .borrow_mut()
            .set_speed(self.panic_speed * them.borrow().dx.signum());
        self.state = KoopaState::Panic;
    }

    fn handle_stomp(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        match self.state {
            KoopaState::Walking => self.hide(us),
            KoopaState::Hiding => {
                us.borrow_mut().dx = 100.;
                us.borrow_mut().dy = -200.;
                event_emitter.borrow().kill(them, us, 100., -200.);
            }
            KoopaState::Panic => {
                self.hide(us);
            }
        };
    }

    fn handle_nudge(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        match self.state {
            KoopaState::Walking => {
                // Killer
                event_emitter.borrow().kill(us, them, 0., -300.);
            }
            KoopaState::Hiding => {
                self.panic(us, them);
            }
            KoopaState::Panic => {
                let travel_dir = us.borrow().dx.signum();
                let delta = us.borrow().x - them.borrow().x;
                let impact_dir = delta.signum();
                if travel_dir != 0. && (travel_dir - impact_dir).abs() > 0. {
                    // Killer
                    event_emitter.borrow().kill(us, them, 0., -300.);
                } else {
                    // FIXME need to move us is the right x
                }
            }
        };
    }
}

impl EntityTrait for KoopaBehavior {
    fn name(&self) -> &str {
        "koopa"
    }
    fn update(&mut self, us: Rc<RefCell<Entity>>, context: &GameContext) {
        if self.state == KoopaState::Hiding {
            self.hide_time += context.dt();
            if self.hide_time > self.hide_duration {
                self.unhide(us);
            }
        }
    }

    fn collides(
        &mut self,
        us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        event_emitter: Rc<RefCell<EventEmitter>>,
    ) {
        if them.borrow().living != Living::Alive {
            return;
        }
        if us.borrow().living != Living::Alive {
            return;
        }
        if them.borrow().is_stomper() {
            if them.borrow().dy > us.borrow().dy {
                self.handle_stomp(us, them, event_emitter);
            } else {
                self.handle_nudge(us, them, event_emitter);
            }
        }
    }
}
