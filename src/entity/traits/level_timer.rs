use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::game::GameContext;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::__rt::core::cell::RefCell;

pub struct LevelTimer {
    current_time: Rc<Cell<f64>>,
    hurry_time: f64,
    hurry_emitted: bool,
}

impl LevelTimer {
    pub fn new(total_time: f64, hurry_time: f64) -> Self {
        let current_time = Rc::new(Cell::new(total_time));
        let hurry_emitted = true;

        Self {
            current_time,
            hurry_time,
            hurry_emitted,
        }
    }

    pub fn current_time(&self) -> Rc<Cell<f64>> {
        self.current_time.clone()
    }
}

impl EntityTrait for LevelTimer {
    fn name(&self) -> &str {
        "timer"
    }

    fn update(&mut self, _entity: Rc<RefCell<Entity>>, context: &GameContext) {
        let mut ct = self.current_time.get();
        ct -= 2. * context.dt();
        self.current_time.set(ct);

        if ct < self.hurry_time && !self.hurry_emitted {
            self.hurry_emitted = true;
            context.emitter().borrow_mut().hurry();
        }

        if ct > self.hurry_time && self.hurry_emitted {
            self.hurry_emitted = false;
            context.emitter().borrow_mut().time_ok();
        }

        if ct < 0. {
            context.emitter().borrow_mut().timeout();
        }
    }
}
