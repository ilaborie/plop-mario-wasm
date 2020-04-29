use crate::assets::TILE_SIZE;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::ObstructionSide;
use crate::physics::bounding_box::BoundingBox;
use crate::physics::matrix::Matrix;
use crate::physics::tile_resolver::TileResolver;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TileCollider {
    resolver: Rc<TileResolver>,
}

impl TileCollider {
    pub fn new(tiles: Rc<RefCell<Matrix<BoundingBox>>>) -> Self {
        let resolver = Rc::new(TileResolver::new(tiles, TILE_SIZE));
        Self { resolver }
    }

    pub fn resolver(&self) -> Rc<TileResolver> {
        self.resolver.clone()
    }

    pub fn check_x(&self, entity: Rc<RefCell<dyn DrawableEntity>>) {
        let dx = entity.borrow().dx();
        if dx == 0.0 {
            return;
        }

        let collision_box = entity.borrow().collision_box();
        let y = collision_box.top();
        let height = collision_box.height();

        let x_test = if dx > 0.0 {
            collision_box.right()
        } else {
            collision_box.left()
        };

        for rect in self.resolver.search_by_range(x_test, y, 0, height as u32) {
            // let dx = entity.borrow().dx();
            let bounding_box = entity.borrow().collision_box();
            // collision
            if dx > 0.0 {
                if bounding_box.right() > rect.left() {
                    // log(&format!("RIGHT, {:?} collide with {:?}", bounding_box, rect).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Right, rect);
                    return;
                }
            } else if dx < 0.0 && bounding_box.left() < rect.right() {
                // log(&format!("LEFT, {:?} collide with {:?}", bounding_box, rect).to_string());
                entity.borrow_mut().obstruct(ObstructionSide::Left, rect);
                return;
            }
        }
    }

    pub fn check_y(&self, entity: Rc<RefCell<dyn DrawableEntity>>) {
        let dy = entity.borrow().dy();
        if dy == 0.0 {
            return;
        }

        let collision_box = entity.borrow().collision_box();
        let x = collision_box.left();
        let width = collision_box.width();

        let y_test = if dy > 0.0 {
            collision_box.bottom()
        } else {
            collision_box.top()
        };
        let tiles = self.resolver.search_by_range(x, y_test, width as u32, 0);
        for &rect in tiles.iter() {
            // let dy = entity.borrow().dy();
            let bbox = entity.borrow().collision_box();

            // Ground collision
            if dy > 0.0 {
                if bbox.bottom() > rect.top() {
                    // log(&format!("DOWN, {:?} collide with {:?}", bbox, rect).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Bottom, rect);
                    return;
                }
            } else if dy < 0.0 && bbox.top() < rect.bottom() {
                // log(&format!("UP, {:?} collide with {:?}", bbox, rect).to_string());
                entity.borrow_mut().obstruct(ObstructionSide::Top, rect);
                return;
            }
        }
    }
}
