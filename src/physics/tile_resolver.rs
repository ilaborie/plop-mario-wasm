use crate::physics::matrix::Matrix;
use crate::physics::rectangle::Rectangle;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

pub struct TileResolver {
    tiles: Rc<RefCell<Matrix<Rectangle>>>,
    tile_size: u32,
}

impl TileResolver {
    pub fn new(tiles: Rc<RefCell<Matrix<Rectangle>>>, tile_size: u32) -> Self {
        Self { tiles, tile_size }
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn to_index(&self, value: f64) -> u32 {
        (value / self.tile_size as f64).floor() as u32
    }
    pub fn to_index_range(&self, pos1: f64, pos2: f64) -> Range<u32> {
        let p_min = (pos1.min(pos2) / self.tile_size as f64).floor() as u32;
        let p_max = (pos1.max(pos2) / self.tile_size as f64).ceil() as u32;
        p_min..p_max
    }

    fn get_by_index(&self, x: u32, y: u32) -> Option<Rectangle> {
        self.tiles.borrow().get(x as usize, y as usize).map(|x| *x)
    }

    pub fn search_by_range(&self, x: f64, y: f64, width: u32, height: u32) -> Vec<Rectangle> {
        let mut result = vec![];
        for x in self.to_index_range(x, x + width as f64) {
            for y in self.to_index_range(y, y + height as f64) {
                if let Some(tile_data) = self.get_by_index(x, y) {
                    result.push(tile_data);
                }
            }
        }
        result
    }
}
