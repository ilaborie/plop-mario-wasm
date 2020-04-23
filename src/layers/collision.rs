use crate::entity::sprite::SpriteEntity;
use crate::layers::level::LevelEntity;
use crate::layers::Drawable;
use crate::physics::tile_resolver::TileResolver;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CollisionLayer {
    player: Rc<RefCell<SpriteEntity>>,
    resolver: Rc<TileResolver>,
}

impl CollisionLayer {
    pub(crate) fn new(level: &LevelEntity, player: Rc<RefCell<SpriteEntity>>) -> Self {
        let resolver = level.tiles_collider().resolver();
        Self { resolver, player }
    }
}

impl Drawable for CollisionLayer {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        // Entity
        let (x, y) = self.player.borrow().position();
        let width = self.player.borrow().width();
        let height = self.player.borrow().height();
        context.set_stroke_style(&JsValue::from("red"));
        context.set_fill_style(&JsValue::from("rgba(128,0,0,.5"));
        context.set_line_width(1.0);
        context.stroke_rect(x, y, width as f64, height as f64);

        // Boxes
        let xs = self.resolver.to_index_range(x, x + width as f64);
        let ys = self.resolver.to_index_range(y, y + height as f64);
        let tile_size = self.resolver.tile_size() as f64;
        for xi in xs {
            for yi in ys.clone() {
                context.set_stroke_style(&JsValue::from("blue"));
                context.set_fill_style(&JsValue::from("rgba(0,0,128,.5"));
                context.set_line_width(1.0);
                context.stroke_rect(
                    xi as f64 * tile_size,
                    yi as f64 * tile_size,
                    tile_size,
                    tile_size,
                );
            }
        }
    }
}
