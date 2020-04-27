use crate::assets::TILE_SIZE;
use crate::entity::{DrawableEntity, ObstructionSide};
use crate::physics::matrix::Matrix;
use crate::physics::rectangle::BoundingBox;
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

        let x_test = if dx > 0.0 { collision_box.right() } else { collision_box.left() };

        for rect in self.resolver.search_by_range(x_test, y, 0, height as u32) {
            let dx = entity.borrow().dx();
            // FIXME might check range with predicted move ?

            // collision
            if dx > 0.0 {
                let x2 = entity.borrow().collision_box().right();
                if x2 as f64 > rect.left() {
                    // log(&format!("RIGHT, {x0:.1}..{x1:.1} collide with {d:?}", x0 = x, x1 = x + width as f64, d = tile_data).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Right, &rect);
                    return;
                }
            } else if dx < 0.0 {
                let x1 = entity.borrow().collision_box().left();
                if x1 < rect.right() {
                    // log(&format!("LEFT, {x0:.1}..{x1:.1} collide with {d:?}", x0 = x, x1 = x + width as f64, d = tile_data).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Left, &rect);
                    return;
                }
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

        let y_test = if dy > 0.0 { collision_box.bottom() } else { collision_box.top() };
        let tiles = self.resolver.search_by_range(x, y_test, width as u32, 0);
        for rect in tiles.iter() {
            let dy = entity.borrow().dy();

            // Ground collision
            if dy > 0.0 {
                let bottom = entity.borrow().collision_box().bottom();
                if bottom > rect.top() {
                    // log(&format!("DOWN, {y0:.1}..{y1:.1} collide with {d:?}", y0 = y, y1 = y + height as f64, d = rect).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Bottom, &rect);
                    return;
                }
            } else if dy < 0.0 {
                let top = entity.borrow().collision_box().top();
                if top < rect.bottom() {
                    // log(&format!("UP, {y0:.1}..{y1:.1} collide with {d:?}", y0 = y, y1 = y + height as f64, d = tile_data).to_string());
                    entity.borrow_mut().obstruct(ObstructionSide::Top, &rect);
                    return;
                }
            }
        }
    }
}
