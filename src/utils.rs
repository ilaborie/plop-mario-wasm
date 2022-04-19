use std::f64;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Window};

use crate::assets::sprites::Rectangle;
use crate::physics::Size;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn request_animation_frame(update: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(update.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn time() -> f64 {
    window()
        .performance()
        .expect("window should have a performance")
        .now()
}

pub fn canvas(size: Size) -> HtmlCanvasElement {
    let Size { width, height } = size;
    let canvas = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    canvas
}

pub fn context_2d(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub fn create_buffer<T>(size: Size, closure: T) -> HtmlCanvasElement
where
    T: FnOnce(CanvasRenderingContext2d),
{
    let buffer = canvas(size);
    let context = context_2d(&buffer);
    closure(context);
    buffer
}

pub fn create_image_buffer(
    image: Rc<HtmlImageElement>,
    rect: &Rectangle,
    mirror: bool,
) -> HtmlCanvasElement {
    let &Rectangle {
        x,
        y,
        width,
        height,
    } = rect;
    let size = Size::new(width, height);
    create_buffer(size, |context| {
        if mirror {
            context.scale(-1., 1.).unwrap();
            context.translate(-(width as f64), 0.).unwrap();
        }
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image.clone(),
                x as f64,
                y as f64,
                width as f64,
                height as f64,
                0.0,
                0.0,
                width as f64,
                height as f64,
            )
            .unwrap()
    })
}
