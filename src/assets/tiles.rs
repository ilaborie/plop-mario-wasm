use crate::assets::animations::AnimationName;
use crate::assets::levels::{TileData, TileType};
use crate::assets::sprites::Sprite;
use crate::physics::bounding_box::BBox;
use crate::physics::Size;

pub struct Ranges {
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
}

impl Ranges {
    pub fn new(values: &[u32]) -> Self {
        let (x0, x1, y0, y1) = match values.len() {
            2 => {
                let x0 = values[0];
                let x1 = x0 + 1;
                let y0 = values[1];
                let y1 = y0 + 1;
                (x0, x1, y0, y1)
            }
            3 => {
                let x0 = values[0];
                let x1 = x0 + values[1];
                let y0 = values[2];
                let y1 = y0 + 1;
                (x0, x1, y0, y1)
            }
            4 => {
                let x0 = values[0];
                let x1 = x0 + values[1];
                let y0 = values[2];
                let y1 = y0 + values[3];
                (x0, x1, y0, y1)
            }
            _ => panic!("Invalid ranges {:?}, expected a size of 2,3,4", values),
        };

        Self { x0, x1, y0, y1 }
    }

    fn coords(&self) -> Vec<(u32, u32)> {
        let mut result = vec![];
        for x in self.x0..self.x1 {
            for y in self.y0..self.y1 {
                result.push((x, y))
            }
        }
        result
    }
}

#[derive(Deserialize)]
pub struct TilesDefinition {
    #[serde(alias = "name")]
    sprite: Option<Sprite>,
    #[serde(alias = "pattern")]
    pattern: Option<String>,
    #[serde(alias = "type")]
    kind: Option<TileType>,
    animation: Option<AnimationName>,
    ranges: Vec<Vec<u32>>,
}

impl TilesDefinition {
    pub fn sprite(&self) -> Option<Sprite> {
        self.sprite
    }

    pub fn pattern(&self) -> Option<String> {
        self.pattern.clone()
    }

    pub fn coords(&self) -> Vec<(u32, u32)> {
        self.ranges
            .iter()
            .map(|v| Ranges::new(v))
            .flat_map(|r| r.coords())
            .collect()
    }

    pub(crate) fn create_tile_data(&self, x: u32, y: u32, tile_size: Size) -> TileData {
        let sprite = self.sprite.unwrap();
        let left = (x * tile_size.width) as f64;
        let top = (y * tile_size.height) as f64;
        let rectangle = BBox::new(top, left, tile_size);
        TileData::new(sprite, (x, y), self.kind, self.animation, rectangle)
    }
}
