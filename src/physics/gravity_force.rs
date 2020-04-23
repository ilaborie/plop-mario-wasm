#[derive(Clone, Copy, Default)]
pub struct GravityForce {
    pub(crate) g: f64,
}

impl GravityForce {
    pub(crate) fn new(g: f64) -> Self {
        Self { g }
    }
}
