use crate::camera::Camera;
use crate::entity::entity_drawable::DrawableEntity;
use crate::entity::Living;
use crate::layers::Drawable;
use crate::level::Level;
use crate::physics::tile_resolver::TileResolver;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CollisionLayer {
    entity: Rc<RefCell<dyn DrawableEntity>>,
    resolver: Rc<TileResolver>,
}

impl CollisionLayer {
    pub(crate) fn new(level: &Level, entity: Rc<RefCell<dyn DrawableEntity>>) -> Self {
        let resolver = level.tiles_collider().resolver();
        Self { resolver, entity }
    }

    fn draw_entity(
        context: &CanvasRenderingContext2d,
        cam_x: f64,
        cam_y: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        context.set_stroke_style(&JsValue::from("red"));
        context.set_fill_style(&JsValue::from("rgba(128,0,0,.5"));
        context.set_line_width(0.5);
        context.stroke_rect(x - cam_x, y - cam_y, width, height);
    }

    fn draw_box(
        context: &CanvasRenderingContext2d,
        cam_x: f64,
        cam_y: f64,
        tile_size: f64,
        xi: u32,
        yi: u32,
    ) {
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

impl Drawable for CollisionLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: &Camera) {
        if self.entity.borrow().entity().borrow().living() != Living::Alive {
            return;
        }

        let (cam_x, cam_y) = camera.position();

        // Entity
        let collision_box = self.entity.borrow().entity().borrow().collision_box();
        let x = collision_box.left();
        let y = collision_box.top();
        let width = collision_box.width();
        let height = collision_box.height();
        CollisionLayer::draw_entity(context, cam_x, cam_y, x, y, width, height);

        // Boxes
        let xs = self.resolver.to_index_range(x, x + width);
        let ys = self.resolver.to_index_range(y, y + height);
        let tile_size = self.resolver.tile_size() as f64;
        for xi in xs {
            for yi in ys.clone() {
                CollisionLayer::draw_box(context, cam_x, cam_y, tile_size, xi, yi);
            }
        }
    }
}
