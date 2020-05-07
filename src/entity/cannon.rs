use crate::audio::sounds::Fx;
use crate::entity::bullet::BulletEntity;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::traits::emitter::Emitter;
use crate::entity::Entity;
use crate::physics::bounding_box::BBox;
use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

const HOLD_FIRE_THRESHOLD: f64 = 30.;

pub struct CannonEntity {
    entity: Rc<RefCell<Entity>>,
}

impl CannonEntity {
    pub fn new(mut entity: Entity) -> Self {
        // Traits
        let emitter = Emitter::new(5.);
        let emitter = Rc::new(RefCell::new(emitter));
        entity.add_trait(emitter.clone());

        let event_buffer = entity.event_buffer.clone();

        let entity = Rc::new(RefCell::new(entity));
        let result = Self { entity };

        let count = Rc::new(Cell::new(0));
        emitter
            .borrow_mut()
            .add_emitter(Box::new(move |source, _player| {
                let (x, y) = source.borrow().position();
                let (player_x, _) = _player.borrow().position();

                let delta_x = player_x - x;
                if delta_x.abs() < HOLD_FIRE_THRESHOLD {
                    return;
                }

                count.set(count.get() + 1);
                let id = format!("Bullet #{}", count.get());
                let size = source.borrow().size;
                let bounding_box = BBox::new(0., 0., size);
                let mut entity = Entity::new(id, bounding_box, size, event_buffer.clone(), None);
                entity.dx = 80. * delta_x.signum();
                entity.x = x;
                entity.y = y;

                let bullet_entity = BulletEntity::new(entity);
                source
                    .borrow_mut()
                    .creation
                    .push((String::from("bullet"), Rc::new(RefCell::new(bullet_entity))));

                source.borrow_mut().play_fx(Fx::Shoot);
                // log(&format!("Emit {}", id));
            }));

        result
    }
}

impl DrawableEntity for CannonEntity {
    fn entity(&self) -> Rc<RefCell<Entity>> {
        self.entity.clone()
    }
}
