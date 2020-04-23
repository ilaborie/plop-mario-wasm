use crate::assets::levels::LevelDefinition;
use crate::assets::sprites::{Sprite, SpriteSheet};
use crate::entity::sprite::SpriteEntity;
use crate::entity::Updatable;
use crate::layers::backgrounds::BackgroundsLayer;
use crate::layers::collision::CollisionLayer;
use crate::layers::{Compositor, Drawable};
use crate::physics::gravity_force::GravityForce;
use crate::physics::matrix::Matrix;
use crate::physics::tile_collider::TileCollider;
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct LevelEntity {
    compositor: Compositor,
    entities: Vec<Rc<RefCell<SpriteEntity>>>,
    tile_collider: Rc<TileCollider>,
    tiles: Rc<RefCell<Matrix<Sprite>>>,
    gravity: GravityForce,
}

impl LevelEntity {
    pub fn new(level_def: LevelDefinition) -> Self {
        let compositor = Compositor::default();
        let entities = vec![];
        let mut tiles = Matrix::new();
        let gravity = GravityForce::new(4000.0);

        for bg in level_def.backgrounds() {
            for range in bg.ranges() {
                for x in range.x() {
                    for y in range.y() {
                        tiles.set(x as usize, y as usize, bg.tile());
                    }
                }
            }
        }
        let tiles = Rc::new(RefCell::new(tiles));
        let tile_collider = Rc::new(TileCollider::new(tiles.clone()));

        Self {
            compositor,
            entities,
            tiles,
            tile_collider,
            gravity,
        }
    }

    pub fn tiles(&self) -> Rc<RefCell<Matrix<Sprite>>> {
        self.tiles.clone()
    }
    pub fn tiles_collider(&self) -> Rc<TileCollider> {
        self.tile_collider.clone()
    }

    pub fn add_background(&mut self, sprites: &SpriteSheet) {
        let bg_layer = BackgroundsLayer::new(self, sprites);
        self.compositor
            .add_layer(Rc::new(move |ctx| bg_layer.draw(ctx)));
    }

    pub fn add_entity(&mut self, entity: Rc<RefCell<SpriteEntity>>) {
        self.entities.push(entity.clone());

        let collision = CollisionLayer::new(&self, entity.clone());
        self.compositor
            .add_layer(Rc::new(move |ctx| collision.draw(ctx)));
    }
}

impl Drawable for LevelEntity {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        self.compositor.draw(context);

        for entity in self.entities.iter() {
            entity.borrow().draw(context);
        }
    }
}

impl Updatable for LevelEntity {
    fn update(&mut self, dt: f64) {
        for entity in self.entities.iter() {
            // log(&format!("Before upd> {:?}", entity.borrow()));
            entity.borrow_mut().update(dt);

            // Position Y
            // log(&format!("Before Y> {:?}", entity.borrow()));
            let (_x, y) = entity.borrow().position();
            let (_dx, dy) = entity.borrow().velocity();
            entity.borrow_mut().set_y(y + dy * dt);
            self.tile_collider.check_y(entity.clone(), dt);

            // Position X
            // log(&format!("Before X> {:?}", entity.borrow()));
            let (x, _y) = entity.borrow().position();
            let (dx, _dy) = entity.borrow().velocity();
            entity.borrow_mut().set_x(x + dx * dt);
            self.tile_collider.check_x(entity.clone());

            // Gravity
            // log(&format!("Before Gravity> {:?}", entity.borrow()));
            let (_dx, dy) = entity.borrow().velocity();
            // log(&format!("dy {}", entity.borrow().velocity().0).to_string());
            entity.borrow_mut().set_dy(dy + self.gravity.g * dt);
        }
    }
}
