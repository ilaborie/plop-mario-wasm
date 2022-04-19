use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::Living;
use crate::layers::Drawable;
use crate::physics::Size;
use crate::utils::{canvas, context_2d};

pub struct EntityLayer {
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
    entity: Rc<RefCell<dyn DrawableEntity>>,
    sprites: Rc<SpriteSheet>,
}

impl EntityLayer {
    pub fn new(entity: Rc<RefCell<dyn DrawableEntity>>, sprites: Rc<SpriteSheet>) -> Self {
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
    fn draw(&mut self, context: Rc<CanvasRenderingContext2d>, camera: &Camera) {
        let (cam_x, cam_y) = camera.position();
        let (x, y) = self.entity.borrow().position();

        // Draw entity to buffer
        let Size { width, height } = self.entity.borrow().size();
        self.buffer_context
            .clear_rect(0., 0., width as f64, height as f64);

        // Sprite or anim
        let removed = self.entity.borrow().living() == Living::NoExistence;
        if !removed {
            if let Some(entity_display) = self.entity.borrow().entity_display() {
                entity_display.draw(&self.buffer_context, 0., 0., &self.sprites);
            }
        }

        // Draw buffer
        context
            .draw_image_with_html_canvas_element(
                &self.buffer,
                (x - cam_x).floor(),
                (y - cam_y).floor(),
            )
            .unwrap();
    }
}
