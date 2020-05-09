use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::player::PlayerEntity;
use crate::entity::traits::level_timer::LevelTimer;
use crate::entity::traits::player_controller::PlayerController;
use crate::entity::{Entity, Living};
use crate::game::PlayerInfo;
use crate::input::ActionHandler;
use crate::physics::bounding_box::BBox;
use crate::physics::{Direction, Position, Size};
use core::cell::RefCell;
use core::fmt;
use core::fmt::{Debug, Formatter};
use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct PlayerEnv {
    entity: Rc<RefCell<Entity>>,
    player: Rc<RefCell<PlayerEntity>>,
    level_timer: Rc<RefCell<LevelTimer>>,
}

impl PlayerEnv {
    pub fn new(player: Rc<RefCell<PlayerEntity>>) -> Self {
        let id = String::from("PlayerController");
        let size = Size::default();
        let bbox = BBox::new(0., 0., size);
        let mut entity = Entity::new(id, bbox, size, None);

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
        self.player.borrow().id()
    }
    pub fn time(&self) -> Rc<Cell<f64>> {
        self.level_timer.borrow().current_time()
    }
    pub fn score(&self) -> Rc<Cell<u32>> {
        self.player.borrow().player_trait().borrow().score()
    }
    pub fn lives(&self) -> Rc<Cell<u32>> {
        self.player.borrow().player_trait().borrow().lives()
    }
    pub fn coins(&self) -> Rc<Cell<u32>> {
        self.player.borrow().player_trait().borrow().coins()
    }
    pub fn position(&self) -> (f64, f64) {
        self.player.borrow().position()
    }

    pub fn update_player(&self, player_info: &PlayerInfo, position: Position) {
        self.time().set(300.);
        self.player.borrow_mut().reset(player_info, position);
    }

    // Control
    fn can_control(&self) -> bool {
        let living = self.player.borrow().entity().borrow().living;
        living == Living::Alive
    }
}

impl DrawableEntity for PlayerEnv {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }
}

impl ActionHandler for PlayerEnv {
    fn name(&self) -> String {
        self.player.borrow().id()
    }

    fn jump_start(&mut self) {
        if !self.can_control() {
            return;
        }

        self.player.borrow_mut().jump_start();
    }

    fn jump_cancel(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().jump_cancel();
    }

    fn start_move(&mut self, direction: Direction) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().start_move(direction);
    }

    fn stop_move(&mut self, direction: Direction) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().stop_move(direction);
    }

    fn start_run(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().start_run();
    }

    fn stop_run(&mut self) {
        if !self.can_control() {
            return;
        }
        self.player.borrow_mut().stop_run();
    }
}

impl Debug for PlayerEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Env for {:?}", self.entity)
    }
}

impl Hash for PlayerEnv {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.borrow().id().hash(state)
    }
}

impl PartialEq for PlayerEnv {
    fn eq(&self, other: &Self) -> bool {
        self.player.borrow().id() == other.player.borrow().id()
    }
}

impl Eq for PlayerEnv {}
