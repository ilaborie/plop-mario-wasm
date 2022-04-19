use core::cell::RefCell;
use std::rc::Rc;

use crate::assets::levels::TriggerDefinition;
use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::events::EventBuffer;
use crate::game::GameContext;
use crate::scene::level::Level;

pub struct TriggerTrait {
    touches: Vec<Rc<RefCell<Entity>>>,
    trigger: TriggerDefinition,
}

impl TriggerTrait {
    pub fn new(trigger: TriggerDefinition) -> Self {
        let touches = vec![];
        Self { trigger, touches }
    }
}

impl EntityTrait for TriggerTrait {
    fn name(&self) -> &str {
        "trigger"
    }

    fn update(&mut self, _entity: Rc<RefCell<Entity>>, context: &GameContext, level: &Level) {
        if !self.touches.is_empty() {
            let player = level.current_player();
            self.touches.clear();
            context
                .emitter()
                .borrow_mut()
                .trigger(self.trigger.clone(), player);
        }
    }

    fn collides(
        &mut self,
        _us: Rc<RefCell<Entity>>,
        them: Rc<RefCell<Entity>>,
        _event_emitter: Rc<RefCell<EventBuffer>>,
    ) {
        self.touches.clear();
        if them.borrow().is_stomper() {
            self.touches.push(them);
        }
    }
}
