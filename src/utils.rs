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

// #[wasm_bindgen]
// pub struct Fps {
//     frames: Vec<f64>,
//     performance: Performance,
//     last_frame_timestamp: f64,
// }

// #[wasm_bindgen]
// pub struct FpsData {
//     latest: f64,
//     min: f64,
//     avg: f64,
//     max: f64,
// }

// #[wasm_bindgen]
// impl FpsData {
//     pub(crate) fn latest(&self) -> f64 {
//         self.latest
//     }
//     pub(crate) fn min(&self) -> f64 {
//         self.min
//     }
//     pub(crate) fn avg(&self) -> f64 {
//         self.avg
//     }
//     pub(crate) fn max(&self) -> f64 {
//         self.max
//     }
// }
//
// #[wasm_bindgen]
// impl Fps {
//     pub(crate) fn new() -> Self {
//         let frames = vec![];
//         let performance = window()
//             .performance()
//             .expect("should have a performance on window");
//         let last_frame_timestamp = performance.now();
//         Self {
//             frames,
//             performance,
//             last_frame_timestamp,
//         }
//     }
//
//     pub(crate) fn update(&mut self) -> FpsData {
//         let now = self.performance.now();
//         let delta = now - self.last_frame_timestamp;
//         self.last_frame_timestamp = now;
//
//         let latest = 1.0 / delta * 1000.0;
//         self.frames.push(latest);
//         if self.frames.len() > 100 {
//             self.frames.remove(0);
//         }
//
//         let mut min = f64::MAX;
//         let mut max = 0.0_f64;
//         let mut sum = 0.0;
//         for &frame in self.frames.iter() {
//             sum += frame;
//             min = min.min(frame);
//             max = max.max(frame);
//         }
//         let avg = sum / self.frames.len() as f64;
//
//         FpsData {
//             latest,
//             min,
//             avg,
//             max,
//         }
//     }
// }
