use crate::assets::levels::Kind;
use crate::assets::TILE_SIZE;
use crate::entity::animation::AnimationEntity;
use crate::physics::matrix::Matrix;
use crate::physics::tile_resolver::{TileData, TileResolver};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TileCollider {
    resolver: Rc<TileResolver>,
}

impl TileCollider {
    pub fn new(tiles: Rc<RefCell<Matrix<TileData>>>) -> Self {
        let resolver = Rc::new(TileResolver::new(tiles.clone(), TILE_SIZE));
        Self { resolver }
    }

    pub fn resolver(&self) -> Rc<TileResolver> {
        self.resolver.clone()
    }

    pub fn check_x(&self, entity: Rc<RefCell<AnimationEntity>>) {
        let (dx, _dy) = entity.borrow().velocity();
        if dx == 0.0 {
            return;
        }

        let (x, y) = entity.borrow().position();
        let width = entity.borrow().width();
        let height = entity.borrow().height();

        let x_test = if dx > 0.0 { x + width as f64 } else { x };

        for tile_data in self.resolver.search_by_range(x_test, y, 0, height) {
            if tile_data.tile != Some(Kind::Ground) {
                continue;
            }
            let (dx, _dy) = entity.borrow().velocity();
            let (x, _y) = entity.borrow().position();
            // Ground collision
            if dx > 0.0 {
                if x + width as f64 > tile_data.left {
                    // log(&format!("RIGHT, {x0:.1}..{x1:.1} collide with {d:?}", x0 = x, x1 = x + width as f64, d = tile_data).to_string());
                    entity.borrow_mut().set_x(tile_data.left - width as f64);
                    entity.borrow_mut().set_dx(0.0);
                    // entity.borrow_mut().stop_move();
                    // FIXME could return
                }
            } else if dx < 0.0 {
                if x < tile_data.right {
                    // log(&format!("LEFT, {x0:.1}..{x1:.1} collide with {d:?}", x0 = x, x1 = x + width as f64, d = tile_data).to_string());
                    entity.borrow_mut().set_x(tile_data.right);
                    entity.borrow_mut().set_dx(0.0);
                    // entity.borrow_mut().stop_move();
                    // FIXME could return
                }
            }
        }
    }

    pub fn check_y(&self, entity: Rc<RefCell<AnimationEntity>>, dt: f64) {
        let (_dx, dy) = entity.borrow().velocity();
        if dy == 0.0 {
            return;
        }

        let (x, y) = entity.borrow().position();
        let width = entity.borrow().width();
        let height = entity.borrow().height();

        let y_test = if dy > 0.0 { y + height as f64 } else { y };
        let tiles = self.resolver.search_by_range(x, y_test, width, 0);
        for tile_data in tiles.iter() {
            if tile_data.tile != Some(Kind::Ground) {
                continue;
            }
            let (_x, y) = entity.borrow().position();
            let (_dx, dy) = entity.borrow().velocity();
            let new_y = y + dy * dt;
            // log(&format!("Check {y:.1}∆{dy:.1} -> {ny:.1}",y=y, dy=dy, ny=new_y).to_string());

            // Ground collision
            if dy > 0.0 {
                let cy = tile_data.top - (height as f64);
                // if y > 160.5 { panic!("WTF check!") }
                if new_y > cy {
                    // log(&format!("DOWN, {y0:.1}..{y1:.1} collide with {d:?}", y0 = y, y1 = y + height as f64, d = tile_data).to_string());
                    entity.borrow_mut().set_y(cy);
                    entity.borrow_mut().set_dy(0.0);
                }
            } else if dy < 0.0 {
                if new_y < tile_data.bottom {
                    // log(&format!("UP, {y0:.1}..{y1:.1} collide with {d:?}", y0 = y, y1 = y + height as f64, d = tile_data).to_string());
                    entity.borrow_mut().set_y(tile_data.bottom);
                    entity.borrow_mut().set_dy(0.0);
                }
            }
        }
    }
}
