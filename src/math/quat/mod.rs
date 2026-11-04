use super::vec::Vector3;

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

    pub fn from_wxyz(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    pub fn from_vector3(v: Vector3) -> Self {
        Self { w: 0.0, x: v.x, y: v.y, z: v.z }
    }

    pub fn conjugate(&self) -> Self {
        Self { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn length(&self) -> f64 {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();

        Self { 
            w: self.w / len, 
            x: self.x / len, 
            y: self.y / len, 
            z: self.z / len
        }
    }
}