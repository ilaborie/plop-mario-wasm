use core::ops::Mul;

#[derive(Deserialize, Clone, Copy, Default, Debug)]
pub struct Size {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Self::Output {
        Size::new(self.width * rhs, self.height * rhs)
    }
}
