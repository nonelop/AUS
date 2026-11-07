use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};

use super::Matrix3;

impl Neg for Matrix3 {
    type Output = Matrix3;
    
    fn neg(self) -> Self::Output {
        Matrix3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign<Matrix3> for Matrix3 {
    fn add_assign(&mut self, rhs: Matrix3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl SubAssign<Matrix3> for Matrix3 {
    fn sub_assign(&mut self, rhs: Matrix3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Matrix3> for f64 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}