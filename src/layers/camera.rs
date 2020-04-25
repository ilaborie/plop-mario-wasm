use crate::camera::Camera;
use crate::layers::Drawable;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CameraLayer {
    camera: Rc<RefCell<Camera>>,
}

impl CameraLayer {
    pub fn new(camera: Rc<RefCell<Camera>>) -> Self {
        Self { camera }
    }
}

impl Drawable for CameraLayer {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: Rc<RefCell<Camera>>) {
        let (x, y) = self.camera.borrow().position();
        let (cam_x, cam_y) = camera.borrow().position();

        let width = self.camera.borrow().width();
        let height = self.camera.borrow().height();
        context.set_stroke_style(&JsValue::from("purple"));
        context.set_fill_style(&JsValue::from("rgba(128,0,0,.5"));
        context.set_line_width(1.);
        context.stroke_rect(x - cam_x, y - cam_y, width as f64, height as f64);
    }
}
