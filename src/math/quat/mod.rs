#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64
}

impl Quaternion {
    pub const ZERO: Self = Self {w: 0.0, x: 0.0, y: 0.0, z: 0.0};
    pub const IDENTITY: Self = Self {w: 1.0, x: 0.0, y: 0.0, z: 0.0};
}