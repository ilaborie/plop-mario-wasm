use crate::assets::SpriteSheet;
use crate::levels::Background;
use crate::utils::create_buffer;
use std::slice::Iter;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct BackgroundsLayer {
    buffer: HtmlCanvasElement,
}

impl BackgroundsLayer {
    pub(crate) fn new(backgrounds: Iter<'_, Background>, sprites: &SpriteSheet) -> Self {
        let buffer = create_buffer(256, 240, |context| {
            for background in backgrounds {
                let ranges = background.ranges();
                for x in ranges.x() {
                    for y in ranges.y() {
                        sprites.draw_tile(&context, background.tile(), x as f64, y as f64);
                    }
                }
            }
        });
        Self { buffer }
    }
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context
            .draw_image_with_html_canvas_element(&self.buffer, 0 as f64, 0 as f64)
            .unwrap();
    }
}
