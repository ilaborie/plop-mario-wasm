use crate::assets::levels::TileData;
use crate::assets::sprites::SpriteSheet;
use crate::assets::TILE_SIZE;
use crate::camera::Camera;
use crate::layers::Drawable;
use crate::physics::matrix::Matrix;
use crate::physics::tile_resolver::TileResolver;
use crate::physics::{Direction, Size};
use crate::utils::{canvas, context_2d};
use core::ops::RangeInclusive;
use std::cell::Cell;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct BackgroundsLayer {
    size: Size,
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
    tiles: Matrix<TileData>,
    sprites: Rc<SpriteSheet>,
    revolver: Rc<TileResolver>,
    range: RangeInclusive<usize>,
    distance: Rc<Cell<f64>>,
}

impl BackgroundsLayer {
    pub(crate) fn new(
        view: Size,
        tiles: Matrix<TileData>,
        sprites: Rc<SpriteSheet>,
        revolver: Rc<TileResolver>,
        distance: Rc<Cell<f64>>,
    ) -> Self {
        let width = (view.width + 1) * TILE_SIZE; // FIXME hide camera buffer
        let height = view.height * TILE_SIZE;
        let size = Size::new(width, height);
        let buffer = canvas(size);
        let buffer_context = context_2d(&buffer);
        let range = 0..=0;

        Self {
            size,
            buffer,
            buffer_context,
            tiles,
            sprites,
            revolver,
            range,
            distance,
        }
    }

    fn redraw(&mut self, range: RangeInclusive<usize>) {
        if self.range != range {
            self.buffer_context
                .clear_rect(0., 0., self.size.width as f64, self.size.height as f64);
            for (x, y, data) in self.tiles.iter() {
                if range.contains(&x) {
                    self.sprites.draw_tile(
                        &self.buffer_context,
                        data.sprite,
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
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: &Camera) {
        let (cam_x, cam_y) = camera.position();
        let draw_width = camera.width() as usize;
        let draw_from = self.revolver.to_index(cam_x) as usize;
        let draw_to = draw_from + draw_width as usize;
        let range = draw_from..=draw_to;

        // Update static tiles
        self.redraw(range.clone());

        // Draw buffer
        context
            .draw_image_with_html_canvas_element(&self.buffer, -cam_x % TILE_SIZE as f64, -cam_y)
            .unwrap();

        // Draw Animations
        let distance = self.distance.get();
        let direction = Direction::Right;
        let tile_size = self.sprites.tile_size();
        for (x, y, data) in self.tiles.iter() {
            if let Some(animation) = data.animation {
                if range.contains(&x) {
                    let ax = (x - *range.start()) * tile_size.width as usize;
                    let ay = y * tile_size.height as usize;
                    self.sprites.draw_tile_animation(
                        &self.buffer_context,
                        animation,
                        ax as f64,
                        ay as f64,
                        distance,
                        direction,
                    );
                }
            }
        }
    }
}
