use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

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

pub(crate) fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub(crate) fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub(crate) fn request_animation_frame(update: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(update.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub(crate) fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub(crate) fn time() -> f64 {
    window()
        .performance()
        .expect("window should have a performance")
        .now()
}

pub(crate) fn canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let canvas = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    canvas
}

pub(crate) fn context_2d(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

pub(crate) fn create_buffer<T>(width: u32, height: u32, closure: T) -> HtmlCanvasElement
where
    T: FnOnce(CanvasRenderingContext2d) -> (),
{
    let buffer = canvas(width, height);
    let context = context_2d(&buffer);
    closure(context);
    buffer
}
