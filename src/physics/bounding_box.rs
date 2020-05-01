use crate::camera::Camera;
use crate::layers::Drawable;
use crate::physics::Size;
use web_sys::CanvasRenderingContext2d;

#[derive(Copy, Clone, Debug)]
pub struct BBox {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

#[allow(dead_code)]
impl BBox {
    pub fn new(x: f64, y: f64, size: Size) -> Self {
        let top = y;
        let left = x;
        let right = x + size.width as f64;
        let bottom = y + size.height as f64;

        Self {
            top,
            left,
            right,
            bottom,
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> BBox {
        let top = self.top + dy;
        let bottom = self.bottom + dy;
        let left = self.left + dx;
        let right = self.right + dx;

        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn size(&self) -> Size {
        let width = self.width();
        let height = self.height();
        Size::new(width as u32, height as u32)
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    pub fn top(&self) -> f64 {
        self.top
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn bottom(&self) -> f64 {
        self.bottom
    }

    pub fn overlaps(&self, other: BBox) -> bool {
        self.bottom > other.top
            && self.top < other.bottom
            && self.left < other.right
            && self.right > other.left
    }
}

impl Drawable for BBox {
    fn draw(&mut self, context: &CanvasRenderingContext2d, camera: &Camera) {
        let (cam_x, cam_y) = camera.position();

        context.stroke_rect(
            self.left - cam_x,
            self.top - cam_y,
            self.width(),
            self.right,
        );
    }
}
