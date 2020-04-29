use crate::assets::config::PlayerDefault;
use crate::assets::sprites::{AnimationName, Sprite};
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::{DrawableEntity, TraitUpdater};
use crate::entity::traits::go::Go;
use crate::entity::traits::jump::Jump;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::stomper::Stomper;
use crate::entity::{Entity, EntityFeature, Living};
use crate::physics::bounding_box::BoundingBox;
use crate::physics::{Direction, Position};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PlayerEntity {
    entity: Rc<RefCell<Entity>>,
    go: Rc<RefCell<Go>>,
    jump: Rc<RefCell<Jump>>,
    stomper: Rc<RefCell<Stomper>>,
    killable: Rc<RefCell<Killable>>,
    // player_controller: Rc<RefCell<PlayerController>>,
}

impl PlayerEntity {
    pub fn new(position: Position, param: &PlayerDefault) -> Self {
        let size = param.size;
        let bounding_box = BoundingBox::new(0., 0., size);
        let mut entity = Entity::new(String::from("Player"), bounding_box, size);
        entity.x = position.x();
        entity.y = position.y();
        let entity = Rc::new(RefCell::new(entity));

        // Traits
        let go = Rc::new(RefCell::new(Go::new(param.motion)));
        let jump = Rc::new(RefCell::new(Jump::new(param.jumping)));
        let stomper = Rc::new(RefCell::new(Stomper::new(param.stomp)));
        let killable = Rc::default();

        // Features
        entity.borrow_mut().features.push(EntityFeature::Stomper);
        entity.borrow_mut().features.push(EntityFeature::Player);

        Self {
            entity,
            go,
            jump,
            stomper,
            killable,
        }
    }

    pub fn jump_start(&mut self) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.jump.borrow_mut().start();
    }
    pub fn jump_cancel(&mut self) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.jump.borrow_mut().cancel();
    }
    pub fn start_move(&mut self, direction: Direction) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.go
            .borrow_mut()
            .move_to(direction, self.jump.borrow().is_jumping());
    }
    pub fn stop_move(&mut self, direction: Direction) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.go
            .borrow_mut()
            .stop(direction, self.jump.borrow().is_jumping());
    }
    pub fn start_run(&mut self) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.go.borrow_mut().start_run();
    }
    pub fn stop_run(&mut self) {
        if self.entity.borrow().living != Living::Alive {
            return;
        }

        self.go.borrow_mut().stop_run();
    }
}

impl DrawableEntity for PlayerEntity {
    fn id(&self) -> String {
        self.entity.borrow().id.clone()
    }

    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }

    fn entity_display(&self) -> EntityDisplay {
        let name = AnimationName::Run;

        if self.entity.borrow().living != Living::Alive {
            return EntityDisplay::sprite(name, Sprite::Dead, self.go.borrow().direction());
        }

        if self.jump.borrow().is_jumping() {
            return EntityDisplay::sprite(name, Sprite::Jump, self.go.borrow().direction());
        }

        let distance = self.go.borrow().distance();
        if distance > 0. {
            let dx = self.entity.borrow().dx;
            let direction = self.go.borrow().direction();
            if (dx > 0. && direction == Direction::Left)
                || (dx < 0. && direction == Direction::Right)
            {
                EntityDisplay::sprite(name, Sprite::Break, self.go.borrow().direction())
            } else {
                EntityDisplay::animation(name, distance, direction)
            }
        } else {
            EntityDisplay::sprite(name, Sprite::Idle, self.go.borrow().direction())
        }
    }

    fn traits(&mut self, mut func: TraitUpdater) {
        func(self.go.clone());
        func(self.jump.clone());
        func(self.killable.clone());
        func(self.stomper.clone());
    }
}
