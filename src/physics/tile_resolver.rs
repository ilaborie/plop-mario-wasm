use crate::assets::sprites::Sprite;
use crate::physics::matrix::Matrix;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

//
#[derive(Debug)]
pub struct TileData {
    pub(crate) tile: Sprite,
    pub(crate) top: f64,
    pub(crate) right: f64,
    pub(crate) bottom: f64,
    pub(crate) left: f64,
}

impl TileData {
    pub fn new(tile: Sprite, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            tile,
            left,
            top,
            right,
            bottom,
        }
    }
}

//
pub struct TileResolver {
    tiles: Rc<RefCell<Matrix<Sprite>>>,
    tile_size: u32,
}

impl TileResolver {
    pub fn new(tiles: Rc<RefCell<Matrix<Sprite>>>, tile_size: u32) -> Self {
        Self { tiles, tile_size }
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn to_index_range(&self, pos1: f64, pos2: f64) -> Range<u32> {
        let p_min = (pos1.min(pos2) / self.tile_size as f64).floor() as u32;
        let p_max = (pos1.max(pos2) / self.tile_size as f64).ceil() as u32;
        p_min..p_max
    }

    fn get_by_index(&self, x: u32, y: u32) -> Option<TileData> {
        self.tiles.borrow().get(x as usize, y as usize).map(|&elt| {
            let left = (x * self.tile_size) as f64;
            let top = (y * self.tile_size) as f64;
            TileData::new(
                elt,
                top,
                left + self.tile_size as f64,
                top + self.tile_size as f64,
                left,
            )
        })
    }

    pub fn search_by_range(&self, x: f64, y: f64, width: u32, height: u32) -> Vec<TileData> {
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
