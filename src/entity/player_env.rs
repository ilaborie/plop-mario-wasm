use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::player_controller::PlayerController;
use crate::entity::Entity;
use crate::physics::bounding_box::BBox;
use crate::physics::{Position, Size};
use core::cell::RefCell;
use std::rc::Rc;

pub struct PlayerEnv {
    entity: Rc<RefCell<Entity>>,
}

impl PlayerEnv {
    pub fn new(player: Rc<RefCell<Entity>>) -> Self {
        let id = String::from("PlayerController");
        let size = Size::default();
        let bbox = BBox::new(0., 0., size);
        let mut entity = Entity::new(id, bbox, size);
        let mut checkpoint = Position::default();
        checkpoint.set_x(8.);
        let checkpoint = Rc::new(RefCell::new(checkpoint));

        // Traits
        let controller = PlayerController::new(player, checkpoint);
        let controller = Rc::new(RefCell::new(controller));

        entity.traits.push(controller);

        let entity = Rc::new(RefCell::new(entity));
        Self { entity }
    }
}

impl DrawableEntity for PlayerEnv {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }

    fn entity_display(&self) -> EntityDisplay {
        unimplemented!()
    }
}
