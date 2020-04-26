use crate::assets::animations::AnimationName;
use crate::assets::sprites::SpriteSheet;
use crate::camera::Camera;
use crate::entity::animation::AnimationEntity;
use crate::layers::Drawable;
use crate::physics::size::Size;
use crate::utils::{canvas, context_2d};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct AnimationLayer {
    buffer: HtmlCanvasElement,
    buffer_context: CanvasRenderingContext2d,
    entity: Rc<RefCell<AnimationEntity>>,
    sprites: Rc<SpriteSheet>,
    animation: AnimationName,
    size: Size,
}

impl AnimationLayer {
    pub fn new(
        entity: Rc<RefCell<AnimationEntity>>,
        sprites: Rc<SpriteSheet>,
        animation: AnimationName,
        size: Size,
    ) -> Self {
        let buffer = canvas(size.width, size.height);
        let buffer_context = context_2d(&buffer);

        Self {
            buffer,
            buffer_context,
            entity,
            sprites,
            animation,
            size,
        }
    }
}

impl Drawable for AnimationLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        let (cam_x, cam_y) = camera.borrow().position();
        let (x, y) = self.entity.borrow().position();
        // log(&format!("{:?}", self.entity.borrow().to_string()));

        // Draw entity to buffer
        let Size { width, height } = self.size;
        self.buffer_context
            .clear_rect(0., 0., width as f64, height as f64);
        self.sprites.draw_entity_animation(
            &self.buffer_context,
            self.animation,
            0.,
            0.,
            self.entity.clone(),
        );

        // Draw buffer
        context
            .draw_image_with_html_canvas_element(&self.buffer, x - cam_x, y - cam_y)
            .unwrap();
    }
}
