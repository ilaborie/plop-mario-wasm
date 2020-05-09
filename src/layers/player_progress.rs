use crate::assets::font::Font;
use crate::assets::sprites::Sprite;
use crate::entity::entity_display::EntityDisplay;
use crate::physics::Size;
use crate::scene::level::Level;
use crate::utils::{canvas, context_2d};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct PlayerProgress {
    font: Rc<Font>,
    size: Size,
    level: Rc<RefCell<Level>>,
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
}

impl PlayerProgress {
    pub fn new(font: Rc<Font>, level: Rc<RefCell<Level>>) -> Self {
        let size = Size::new(32, 32);
        let buffer = canvas(size);
        let buffer_context = context_2d(&buffer);

        Self {
            level,
            font,
            size,
            buffer,
            buffer_context,
        }
    }

    pub fn draw(&mut self, context: Rc<CanvasRenderingContext2d>) {
        let size = self.font.size() as f64;

        let canvas = context.canvas().unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        // Black
        context.set_fill_style(&JsValue::from("black"));
        context.set_line_width(0.5);
        context.stroke_rect(0., 0., width, height);

        // World
        let lvl = format!("WORLD {}", self.level.borrow().name());
        self.font
            .print(context.clone(), lvl.as_str(), size * 12., size * 12.);

        if let Some(player_env) = self.level.borrow().find_player() {
            let lives = format!("x {}", player_env.borrow().lives().get());
            self.font
                .print(context.clone(), lives.as_str(), size * 16., size * 16.);

            self.buffer_context
                .clear_rect(0., 0., self.size.width as f64, self.size.height as f64);
            // player sprite
            let entity_display = EntityDisplay::sprite(Sprite::Idle);
            let sprite_sheet = self
                .level
                .borrow()
                .sprite_sheet(player_env.borrow().name().as_str());
            entity_display.draw(&self.buffer_context, 0., 0., &sprite_sheet);

            context
                .draw_image_with_html_canvas_element(&self.buffer, size * 12., size * 15.)
                .unwrap();
        }
    }
}
