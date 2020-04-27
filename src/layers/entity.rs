use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::DrawableEntity;
use crate::layers::Drawable;
use crate::physics::size::Size;
use crate::utils::{canvas, context_2d};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct EntityLayer {
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
    entity: Rc<RefCell<dyn DrawableEntity>>,
    sprites: SpriteSheet,
}

impl EntityLayer {
    pub fn new(entity: Rc<RefCell<dyn DrawableEntity>>, sprites: SpriteSheet) -> Self {
        let size = entity.borrow().size();
        let buffer = canvas(size);
        let buffer_context = context_2d(&buffer);

        Self {
            buffer,
            buffer_context,
            entity,
            sprites,
        }
    }
}

impl Drawable for EntityLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        let (cam_x, cam_y) = camera.borrow().position();
        let (x, y) = self.entity.borrow().position();
        // log(&format!("{:?}", self.entity.borrow().to_string()));

        // Draw entity to buffer
        let Size { width, height } = self.entity.borrow().size();
        self.buffer_context
            .clear_rect(0., 0., width as f64, height as f64);

        // Sprite or anim
        let entity_display = self.entity.borrow().entity_display();
        entity_display.draw(&self.buffer_context, 0., 0., &self.sprites);

        // Draw buffer
        context
            .draw_image_with_html_canvas_element(&self.buffer, x - cam_x, y - cam_y)
            .unwrap();
    }
}
