use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::game::GameContext;
use crate::scene::level::Level;
use core::cell::RefCell;
use std::rc::Rc;

type EntityEmitter = Box<dyn Fn(Rc<RefCell<Entity>>, &Level) -> ()>;

pub struct Emitter {
    interval: f64,
    cool_down: f64,
    emitters: Vec<EntityEmitter>,
}

impl Emitter {
    pub fn new(interval: f64) -> Self {
        let cool_down = interval;
        let emitters = vec![];

        Self {
            interval,
            cool_down,
            emitters,
        }
    }

    pub fn add_emitter(&mut self, emitter: EntityEmitter) {
        self.emitters.push(emitter);
    }

    fn emit(&self, entity: Rc<RefCell<Entity>>, level: &Level) {
        for emitter in self.emitters.iter() {
            emitter(entity.clone(), level);
        }
    }
}

impl EntityTrait for Emitter {
    fn name(&self) -> &str {
        "emitter"
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext, level: &Level) {
        self.cool_down -= context.dt();
        if self.cool_down <= 0. {
            self.emit(entity, level);
            self.cool_down = self.interval;
        }
    }
}
