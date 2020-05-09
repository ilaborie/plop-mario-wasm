use crate::assets::animations::AnimationName;
use crate::assets::audio::sounds::AudioBoard;
use crate::assets::config::PlayerDefault;
use crate::assets::sprites::Sprite;
use crate::entity::entity_display::EntityDisplay;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::go::Go;
use crate::entity::traits::jump::Jump;
use crate::entity::traits::killable::Killable;
use crate::entity::traits::physics::Physics;
use crate::entity::traits::player::PlayerTrait;
use crate::entity::traits::solid::Solid;
use crate::entity::traits::stomper::Stomper;
use crate::entity::{Entity, EntityFeature, Living};
use crate::game::PlayerInfo;
use crate::physics::bounding_box::BBox;
use crate::physics::{Direction, Position};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PlayerEntity {
    entity: Rc<RefCell<Entity>>,
    go: Rc<RefCell<Go>>,
    jump: Rc<RefCell<Jump>>,
    player_trait: Rc<RefCell<PlayerTrait>>,
}

impl PlayerEntity {
    pub fn new(
        player_info: &PlayerInfo,
        position: Position,
        param: &PlayerDefault,
        physics: Physics,
        audio: Option<Rc<AudioBoard>>,
    ) -> Self {
        let size = param.size;
        let bounding_box = BBox::new(0., 0., size);
        let mut entity = Entity::new(String::from(player_info.name()), bounding_box, size, audio);
        entity.x = position.x();
        entity.y = position.y();

        // Traits
        let solid = Rc::new(RefCell::new(Solid::new()));
        let go = Rc::new(RefCell::new(Go::new(param.motion)));
        let jump = Rc::new(RefCell::new(Jump::new(param.jumping)));
        let stomper = Rc::new(RefCell::new(Stomper::new()));
        let killable = Rc::new(RefCell::new(Killable::new(solid.clone(), 50., -200.)));
        let physics = Rc::new(RefCell::new(physics));
        let player_trait = PlayerTrait::new(player_info);
        let player_trait = Rc::new(RefCell::new(player_trait));

        entity.add_trait(solid);
        entity.add_trait(go.clone());
        entity.add_trait(jump.clone());
        entity.add_trait(stomper);
        entity.add_trait(killable);
        entity.add_trait(physics);
        entity.add_trait(player_trait.clone());

        // Features
        entity.features.push(EntityFeature::Stomper);
        entity.features.push(EntityFeature::Player);

        let entity = Rc::new(RefCell::new(entity));
        Self {
            entity,
            go,
            jump,
            player_trait,
        }
    }

    pub fn reset(&mut self, player_info: &PlayerInfo, position: Position) {
        self.entity.borrow_mut().set_x(position.x(), 0.);
        self.entity.borrow_mut().set_y(position.y(), 0.);
        self.go.borrow_mut().reset();
        self.jump.borrow_mut().reset();
        self.player_trait.borrow_mut().reset(player_info);
    }

    pub fn player_trait(&self) -> Rc<RefCell<PlayerTrait>> {
        self.player_trait.clone()
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

    fn entity_display(&self) -> Option<EntityDisplay> {
        let name = AnimationName::Run;

        if self.entity.borrow().living != Living::Alive {
            return Some(EntityDisplay::sprite(Sprite::Dead));
        }

        if self.jump.borrow().is_jumping() {
            return Some(EntityDisplay::sprite_direction(
                name,
                Sprite::Jump,
                self.go.borrow().direction(),
            ));
        }

        let distance = self.go.borrow().distance();
        let result = if distance > 0. {
            let dx = self.entity.borrow().dx;
            let direction = self.go.borrow().direction();
            if (dx > 0. && direction == Direction::Left)
                || (dx < 0. && direction == Direction::Right)
            {
                EntityDisplay::sprite_direction(name, Sprite::Break, self.go.borrow().direction())
            } else {
                EntityDisplay::animation(name, distance, direction)
            }
        } else {
            EntityDisplay::sprite_direction(name, Sprite::Idle, self.go.borrow().direction())
        };
        Some(result)
    }
}
