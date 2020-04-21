use web_sys::CanvasRenderingContext2d;

pub mod backgrounds;
pub mod sprite;

pub type Layer = dyn Fn(&CanvasRenderingContext2d) -> ();
