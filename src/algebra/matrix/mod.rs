pub mod ops;

use super::vector::Vector3;

pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3
}

impl Matrix3 {
    pub const ZERO: Self = Self { 
        x: Vector3::ZERO, 
        y: Vector3::ZERO,
        z: Vector3::ZERO
    };
}
