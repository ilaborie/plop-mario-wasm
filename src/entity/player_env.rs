use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::player::PlayerEntity;
use crate::entity::traits::player_controller::PlayerController;
use crate::entity::{Entity, Living};
use crate::physics::bounding_box::BBox;
use crate::physics::{Direction, Position, Size};
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

pub struct PlayerEnv {
    entity: Rc<RefCell<Entity>>,
    player: Rc<RefCell<PlayerEntity>>,
    time: Rc<Cell<f64>>,
    coins: Rc<Cell<u32>>,
}

impl PlayerEnv {
    pub fn new(player: Rc<RefCell<PlayerEntity>>) -> Self {
        let id = String::from("PlayerController");
        let size = Size::default();
        let bbox = BBox::new(0., 0., size);
        let mut entity = Entity::new(id, bbox, size);

        let mut checkpoint = Position::default();
        checkpoint.set_x(8.);
        let checkpoint = Rc::new(RefCell::new(checkpoint));

        let time = Rc::new(Cell::new(300.));
        let coins = Rc::new(Cell::new(0));

        // Traits
        let controller = PlayerController::new(player.borrow().entity(), time.clone(), checkpoint);
        let controller = Rc::new(RefCell::new(controller));
        entity.traits.push(controller);

        let entity = Rc::new(RefCell::new(entity));
        Self {
            player,
            entity,
            time,
            coins,
        }
    }

    // Player info
    pub fn name(&self) -> String {
        self.player.borrow().id().to_uppercase()
    }
    pub fn score(&self) -> Rc<Cell<u32>> {
        self.player.borrow().entity().borrow().score()
    }
    pub fn time(&self) -> Rc<Cell<f64>> {
        self.time.clone()
    }
    pub fn coins(&self) -> Rc<Cell<u32>> {
        self.coins.clone()
    }
    pub fn position(&self) -> (f64, f64) {
        self.player.borrow().position()
    }

    // Control
    fn can_control(&self) -> bool {
        let living = self.player.borrow().entity().borrow().living;
        living == Living::Alive
    }

    pub fn jump_start(&mut self) {
        if !self.can_control() {
            return;
        }

        self.player.borrow_mut().jump_start();
    }
    pub fn jump_cancel(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().jump_cancel();
    }
    pub fn start_move(&mut self, direction: Direction) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().start_move(direction);
    }
    pub fn stop_move(&mut self, direction: Direction) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().stop_move(direction);
    }
    pub fn start_run(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().start_run();
    }
    pub fn stop_run(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().stop_run();
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
