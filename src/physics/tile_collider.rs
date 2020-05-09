use crate::assets::levels::{TileData, TileType};
use crate::assets::sprites::Sprite;
use crate::assets::TILE_SIZE;
use crate::entity::traits::obstruct;
use crate::entity::{Entity, Living, ObstructionSide};
use crate::events::EventBuffer;
use crate::physics::bounding_box::BBox;
use crate::physics::matrix::Matrix;
use crate::physics::tile_resolver::TileResolver;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TileCollider {
    resolvers: Vec<TileResolver>,
}

impl TileCollider {
    pub fn new(tiles: &[Rc<RefCell<Matrix<TileData>>>]) -> Self {
        let resolvers = tiles
            .iter()
            .map(|mat| TileResolver::new(mat.clone(), TILE_SIZE))
            .collect();
        Self { resolvers }
    }

    pub fn check_x(&mut self, entity: Rc<RefCell<Entity>>, event_buffer: Rc<RefCell<EventBuffer>>) {
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

        for resolver in self.resolvers.iter_mut() {
            for tile_data in resolver.search_by_range(x_test, y, 0, height as u32) {
                if let Some(tile) = tile_data.tile() {
                    tile.handle_x(entity.clone(), &tile_data, resolver, event_buffer.clone())
                }
            }
        }
    }

    pub fn check_y(&mut self, entity: Rc<RefCell<Entity>>, event_buffer: Rc<RefCell<EventBuffer>>) {
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

        for resolver in self.resolvers.iter_mut() {
            let tiles = resolver.search_by_range(x, y_test, width as u32, 0);
            for tile_data in tiles.iter() {
                if let Some(tile) = tile_data.tile() {
                    tile.handle_y(entity.clone(), tile_data, resolver, event_buffer.clone())
                }
            }
        }
    }
}

impl TileType {
    fn handle_x(
        self,
        entity: Rc<RefCell<Entity>>,
        tile_data: &TileData,
        resolver: &mut TileResolver,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        if let TileType::Coin = self {
            TileType::handle_coin(entity, tile_data, resolver, event_buffer)
        } else {
            TileType::handle_solid_x(entity, tile_data.rectangle());
        }
    }

    fn handle_y(
        self,
        entity: Rc<RefCell<Entity>>,
        tile_data: &TileData,
        resolver: &mut TileResolver,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        match self {
            TileType::Ground => TileType::handle_solid_y(entity, tile_data.rectangle()),
            TileType::Brick => TileType::handle_brick_y(entity, tile_data, resolver),
            TileType::BrickBroken => TileType::handle_brick_y(entity, tile_data, resolver),
            TileType::Coin => TileType::handle_coin(entity, tile_data, resolver, event_buffer),
        }
    }

    fn handle_solid_x(entity: Rc<RefCell<Entity>>, rect: BBox) {
        let bbox = entity.borrow().collision_box();
        let dx = entity.borrow().dx();
        if dx > 0.0 {
            if bbox.right() > rect.left() {
                obstruct(entity, ObstructionSide::Right, rect);
            }
        } else if dx < 0.0 && bbox.left() < rect.right() {
            obstruct(entity, ObstructionSide::Left, rect);
        }
    }

    fn handle_solid_y(entity: Rc<RefCell<Entity>>, rect: BBox) {
        let bbox = entity.borrow().collision_box();
        let dy = entity.borrow().dy();
        if dy > 0.0 {
            if bbox.bottom() > rect.top() {
                obstruct(entity, ObstructionSide::Bottom, rect);
            }
        } else if dy < 0.0 && bbox.top() < rect.bottom() {
            obstruct(entity, ObstructionSide::Top, rect);
        }
    }
    fn handle_brick_y(
        entity: Rc<RefCell<Entity>>,
        tile_data: &TileData,
        resolver: &mut TileResolver,
    ) {
        if entity.borrow().living() != Living::Alive {
            return;
        }
        let bbox = entity.borrow().collision_box();
        let dy = entity.borrow().dy();
        let rect = tile_data.rectangle();
        if dy > 0.0 {
            if bbox.bottom() > rect.top() {
                obstruct(entity, ObstructionSide::Bottom, rect);
            }
        } else if dy < 0.0 && bbox.top() < rect.bottom() {
            obstruct(entity, ObstructionSide::Top, rect);
            if tile_data.sprite() == Sprite::Brick {
                let td = tile_data.replace_sprite(Sprite::BrickBroken);
                resolver.update(td);
            } else {
                resolver.remove(tile_data);
            }
        }
    }

    fn handle_coin(
        entity: Rc<RefCell<Entity>>,
        tile_data: &TileData,
        resolver: &mut TileResolver,
        event_buffer: Rc<RefCell<EventBuffer>>,
    ) {
        event_buffer.borrow_mut().coin(entity.borrow().id(), 1);
        resolver.remove(tile_data);
    }
}
