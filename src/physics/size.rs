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
