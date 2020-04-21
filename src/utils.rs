use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
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


pub(crate) fn create_buffer<T>(width: u32, height: u32, closure: T) -> HtmlCanvasElement
    where T: FnOnce(CanvasRenderingContext2d) -> () {
    let buffer = document().create_element("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    buffer.set_width(width);
    buffer.set_height(height);

    let context = buffer.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    closure(context);

    buffer
}
