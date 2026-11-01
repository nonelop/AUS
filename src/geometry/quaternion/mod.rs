#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64
}

impl Quaternion {
    pub fn zero() -> Self {
        Quaternion { w: 0.0, x: 0.0, y: 0.0, z: 0.0 }
    }
}