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

        self.buffer_context
            .clear_rect(0., 0., self.size.width as f64, self.size.height as f64);
        // Draw entity to buffer
        let dir = self.entity.borrow().direction();
        let dist = self.entity.borrow().distance();
        self.sprites
            .draw_animation(&self.buffer_context, self.animation, 0., 0., dir, dist);

        context
            .draw_image_with_html_canvas_element(&self.buffer, x - cam_x, y - cam_y)
            .unwrap();
    }
}
