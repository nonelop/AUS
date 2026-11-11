pub mod ops;

#[cfg(test)]
mod tests;

use super::vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
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

    pub fn from_rotation_x(angle: f64) -> Self {
        let half_angle = angle / 2.0;
        Self { w: half_angle.cos(), x: half_angle.sin(), y: 0.0, z: 0.0 }
    }

    pub fn from_rotation_y(angle: f64) -> Self {
        let half_angle = angle / 2.0;
        Self { w: half_angle.cos(), x: 0.0, y: half_angle.sin(), z: 0.0 }
    }

    pub fn from_rotation_z(angle: f64) -> Self {
        let half_angle = angle / 2.0;
        Self { w: half_angle.cos(), x: 0.0, y: 0.0, z: half_angle.sin() }
    }

    pub fn from_axis(axis: Vector3, angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let axis = axis.normalize();

        Self { 
            w: half_angle.cos(), 
            x: axis.x * half_angle.sin(), 
            y: axis.y * half_angle.sin(),
            z: axis.z * half_angle.sin()
        }
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