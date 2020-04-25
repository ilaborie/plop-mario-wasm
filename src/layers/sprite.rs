// use crate::assets::sprites::SpriteSheet;
// use crate::camera::Camera;
// use crate::entity::sprite::SpriteEntity;
// use crate::layers::Drawable;
// use crate::physics::size::Size;
// use crate::utils::{canvas, context_2d};
// use std::cell::RefCell;
// use std::rc::Rc;
// use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
//
// pub struct SpriteLayer {
//     buffer: HtmlCanvasElement,
//     buffer_context: CanvasRenderingContext2d,
//     entity: Rc<RefCell<SpriteEntity>>,
//     sprites: Rc<SpriteSheet>,
//     size: Size,
// }
//
// impl SpriteLayer {
//     pub(crate) fn new(
//         entity: Rc<RefCell<SpriteEntity>>,
//         sprites: Rc<SpriteSheet>,
//         size: Size,
//     ) -> Self {
//         let buffer = canvas(size.width, size.height);
//         let buffer_context = context_2d(&buffer);
//
//         Self {
//             buffer,
//             buffer_context,
//             entity,
//             sprites,
//             size,
//         }
//     }
// }
//
// impl Drawable for SpriteLayer {
//     fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
//         let (cam_x, cam_y) = camera.borrow().position();
//         let (x, y) = self.entity.borrow().position();
//
//         self.buffer_context
//             .clear_rect(0., 0., self.size.width as f64, self.size.height as f64);
//         // Draw entity to buffer
//         self.sprites
//             .draw_tile(&self.buffer_context, &self.entity.borrow().sprite(), 0., 0.);
//
//         context
//             .draw_image_with_html_canvas_element(&self.buffer, x - cam_x, y - cam_y)
//             .unwrap();
//     }
// }
