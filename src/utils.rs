use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Performance, Window};

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
where
    T: FnOnce(CanvasRenderingContext2d) -> (),
{
    let buffer = document()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    buffer.set_width(width);
    buffer.set_height(height);

    let context = buffer
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    closure(context);

    buffer
}

#[wasm_bindgen]
pub struct Fps {
    frames: Vec<f64>,
    performance: Performance,
    last_frame_timestamp: f64,
}

#[wasm_bindgen]
pub struct FpsData {
    latest: f64,
    min: f64,
    avg: f64,
    max: f64,
}

#[wasm_bindgen]
impl FpsData {
    pub fn latest(&self) -> f64 {
        self.latest
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn avg(&self) -> f64 {
        self.avg
    }
    pub fn max(&self) -> f64 {
        self.max
    }
}

#[wasm_bindgen]
impl Fps {
    pub fn new() -> Self {
        let frames = vec![];
        let performance = window()
            .performance()
            .expect("should have a performance on window");
        let last_frame_timestamp = performance.now();
        Self {
            frames,
            performance,
            last_frame_timestamp,
        }
    }

    pub fn update(&mut self) -> FpsData {
        let now = self.performance.now();
        let delta = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;

        let latest = 1.0 / delta * 1000.0;
        self.frames.push(latest);
        if self.frames.len() > 100 {
            self.frames.remove(0);
        }

        let mut min = f64::MAX;
        let mut max = 0.0_f64;
        let mut sum = 0.0;
        for &frame in self.frames.iter() {
            sum += frame;
            min = min.min(frame);
            max = max.max(frame);
        }
        let avg = sum / self.frames.len() as f64;

        FpsData {
            latest,
            min,
            avg,
            max,
        }
    }
}
