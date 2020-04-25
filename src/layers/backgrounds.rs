use crate::assets::sprites::SpriteSheet;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::layers::Drawable;
use crate::physics::matrix::Matrix;
use crate::physics::tile_resolver::{TileData, TileResolver};
use crate::utils::{canvas, context_2d};
use core::ops::RangeInclusive;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct BackgroundsLayer {
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
    tiles: Rc<RefCell<Matrix<TileData>>>,
    sprites: SpriteSheet,
    revolver: Rc<TileResolver>,
    range: RangeInclusive<usize>,
}

impl BackgroundsLayer {
    pub(crate) fn new(
        tiles: Rc<RefCell<Matrix<TileData>>>,
        sprites: SpriteSheet,
        revolver: Rc<TileResolver>,
    ) -> Self {
        let width = 17 * TILE_SIZE;
        let height = 15 * TILE_SIZE;
        let buffer = canvas(width, height);
        let buffer_context = context_2d(&buffer);
        let range = 0..=0;

        Self {
            buffer,
            buffer_context,
            tiles,
            sprites,
            revolver,
            range,
        }
    }

    fn redraw(&mut self, range: RangeInclusive<usize>) {
        if self.range != range {
            for (x, y, data) in self.tiles.borrow().iter() {
                if range.contains(&x) {
                    self.sprites.draw_tile(
                        &self.buffer_context,
                        &data.sprite,
                        (x - *range.start()) as f64,
                        y as f64,
                    );
                }
            }
            self.range = range;
        }
    }
}

impl Drawable for BackgroundsLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        let (cam_x, cam_y) = camera.borrow().position();
        let draw_width = camera.borrow().width() as usize;
        let draw_from = self.revolver.to_index(cam_x) as usize;
        let draw_to = draw_from + draw_width as usize;
        let range = draw_from..=draw_to;

        self.redraw(range);

        context
            .draw_image_with_html_canvas_element(&self.buffer, -cam_x % TILE_SIZE as f64, -cam_y)
            .unwrap();
    }
}
