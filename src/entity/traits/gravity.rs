use crate::entity::traits::EntityTrait;
use crate::entity::Entity;
use crate::game::GameContext;
use crate::physics::GravityForce;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;

pub struct Gravity {
    g: GravityForce,
}

#[allow(dead_code)]
impl Gravity {
    pub fn new(g: GravityForce) -> Self {
        Self { g }
    }
}

impl EntityTrait for Gravity {
    fn name(&self) -> &str {
        "gravity"
    }

    fn update(&mut self, entity: Rc<RefCell<Entity>>, context: &GameContext) {
        entity.borrow_mut().dy += self.g.g * context.dt();
    }
}
