use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::assets::levels::TileData;
use crate::physics::matrix::Matrix;

pub struct TileResolver {
    tiles: Rc<RefCell<Matrix<TileData>>>,
    tile_size: u32,
}

impl TileResolver {
    pub fn new(tiles: Rc<RefCell<Matrix<TileData>>>, tile_size: u32) -> Self {
        Self { tiles, tile_size }
    }

    pub fn index(tile_size: u32, value: f64) -> u32 {
        (value / tile_size as f64).floor() as u32
    }
    pub fn index_range(tile_size: u32, pos1: f64, pos2: f64) -> Range<u32> {
        let p_min = (pos1.min(pos2) / tile_size as f64).floor() as u32;
        let p_max = (pos1.max(pos2) / tile_size as f64).ceil() as u32;
        p_min..p_max
    }

    pub fn update(&mut self, tile_data: TileData) {
        let (x, y) = tile_data.position();
        self.tiles
            .borrow_mut()
            .set(x as usize, y as usize, tile_data);
    }

    pub fn remove(&mut self, tile_data: &TileData) {
        let (x, y) = tile_data.position();
        self.tiles.borrow_mut().remove(x as usize, y as usize);
    }

    fn get_by_index(&self, x: u32, y: u32) -> Option<TileData> {
        self.tiles.borrow().get(x as usize, y as usize).copied()
    }

    pub fn search_by_range(&self, x: f64, y: f64, width: u32, height: u32) -> Vec<TileData> {
        let mut result = vec![];
        for x in TileResolver::index_range(self.tile_size, x, x + width as f64) {
            for y in TileResolver::index_range(self.tile_size, y, y + height as f64) {
                if let Some(tile_data) = self.get_by_index(x, y) {
                    result.push(tile_data);
                }
            }
        }
        result
    }
}
