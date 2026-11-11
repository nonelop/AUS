pub mod ops;

#[cfg(test)]
mod tests;

use super::vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub c1: Vector3,
    pub c2: Vector3,
    pub c3: Vector3
}

impl Matrix3 {
    pub const ZERO: Self = Self { 
        c1: Vector3::ZERO, 
        c2: Vector3::ZERO,
        c3: Vector3::ZERO
    };
}