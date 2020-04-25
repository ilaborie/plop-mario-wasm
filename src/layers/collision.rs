use crate::camera::Camera;
use crate::entity::animation::AnimationEntity;
use crate::layers::Drawable;
use crate::level::Level;
use crate::physics::tile_resolver::TileResolver;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CollisionLayer {
    player: Rc<RefCell<AnimationEntity>>,
    resolver: Rc<TileResolver>,
}

impl CollisionLayer {
    pub(crate) fn new(level: &Level, player: Rc<RefCell<AnimationEntity>>) -> Self {
        let resolver = level.tiles_collider().resolver();
        Self { resolver, player }
    }
}

impl Drawable for CollisionLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        let (cam_x, cam_y) = camera.borrow().position();

        // Entity
        let (x, y) = self.player.borrow().position();
        let width = self.player.borrow().width();
        let height = self.player.borrow().height();
        context.set_stroke_style(&JsValue::from("red"));
        context.set_fill_style(&JsValue::from("rgba(128,0,0,.5"));
        context.set_line_width(0.5);
        context.stroke_rect(x - cam_x, y - cam_y, width as f64, height as f64);

        // Boxes
        let xs = self.resolver.to_index_range(x, x + width as f64);
        let ys = self.resolver.to_index_range(y, y + height as f64);
        let tile_size = self.resolver.tile_size() as f64;
        for xi in xs {
            for yi in ys.clone() {
                context.set_stroke_style(&JsValue::from("blue"));
                context.set_fill_style(&JsValue::from("rgba(0,0,128,.5"));
                context.set_line_width(0.5);
                context.stroke_rect(
                    xi as f64 * tile_size - cam_x,
                    yi as f64 * tile_size - cam_y,
                    tile_size,
                    tile_size,
                );
            }
        }
    }
}
