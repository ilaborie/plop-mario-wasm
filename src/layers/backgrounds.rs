use crate::assets::sprites::SpriteSheet;
use crate::layers::level::LevelEntity;
use crate::layers::Drawable;
use crate::utils::create_buffer;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct BackgroundsLayer {
    buffer: HtmlCanvasElement,
}

impl BackgroundsLayer {
    pub(crate) fn new(level: &LevelEntity, sprites: &SpriteSheet) -> Self {
        let buffer = create_buffer(256, 240, |context| {
            for (x, y, sprite) in level.tiles().borrow().iter() {
                sprites.draw_tile(&context, sprite, x as f64, y as f64);
            }
        });
        Self { buffer }
    }
}

impl Drawable for BackgroundsLayer {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        context
            .draw_image_with_html_canvas_element(&self.buffer, 0 as f64, 0 as f64)
            .unwrap();
    }
}
