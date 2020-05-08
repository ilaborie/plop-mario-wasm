use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::events::EventBuffer;
use crate::entity::player::PlayerEntity;
use crate::entity::traits::level_timer::LevelTimer;
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
    level_timer: Rc<RefCell<LevelTimer>>,
}

impl PlayerEnv {
    pub fn new(player: Rc<RefCell<PlayerEntity>>, event_buffer: Rc<RefCell<EventBuffer>>) -> Self {
        let id = String::from("PlayerController");
        let size = Size::default();
        let bbox = BBox::new(0., 0., size);
        let mut entity = Entity::new(id, bbox, size, event_buffer, None);

        let mut checkpoint = Position::default();
        checkpoint.set_x(8.);
        let checkpoint = Rc::new(RefCell::new(checkpoint));

        // Traits
        let controller = PlayerController::new(player.clone().borrow().entity(), checkpoint);
        let controller = Rc::new(RefCell::new(controller));
        let level_timer = LevelTimer::new(300., 100.);
        let level_timer = Rc::new(RefCell::new(level_timer));
        entity.add_trait(controller);
        entity.add_trait(level_timer.clone());

        let entity = Rc::new(RefCell::new(entity));
        Self {
            player,
            entity,
            level_timer,
        }
    }

    // Player info
    pub fn name(&self) -> String {
        self.player.borrow().id().to_uppercase()
    }
    pub fn time(&self) -> Rc<Cell<f64>> {
        self.level_timer.borrow().current_time()
    }
    pub fn score(&self) -> Rc<Cell<u32>> {
        self.player.borrow().player_trait().borrow().score()
    }
    pub fn coins(&self) -> Rc<Cell<u32>> {
        self.player.borrow().player_trait().borrow().coins()
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
}
