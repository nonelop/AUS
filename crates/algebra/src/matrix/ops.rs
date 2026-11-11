use super::Matrix3;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use std::fmt;

impl Neg for Matrix3 {
    type Output = Matrix3;
    
    fn neg(self) -> Self::Output {
        Matrix3 {
            c1: -self.c1,
            c2: -self.c2,
            c3: -self.c3
        }
    }
}

impl Add<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            c1: self.c1 + rhs.c1,
            c2: self.c2 + rhs.c2,
            c3: self.c3 + rhs.c3
        }
    }
}

impl AddAssign<Matrix3> for Matrix3 {
    fn add_assign(&mut self, rhs: Matrix3) {
        self.c1 += rhs.c1;
        self.c2 += rhs.c2;
        self.c3 += rhs.c3;
    }
}

impl Sub<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            c1: self.c1 - rhs.c1,
            c2: self.c2 - rhs.c2,
            c3: self.c3 - rhs.c3
        }
    }
}

impl SubAssign<Matrix3> for Matrix3 {
    fn sub_assign(&mut self, rhs: Matrix3) {
        self.c1 -= rhs.c1;
        self.c2 -= rhs.c2;
        self.c3 -= rhs.c3;
    }
}

impl Mul<f64> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3 {
            c1: self.c1 * rhs,
            c2: self.c2 * rhs,
            c3: self.c3 * rhs
        }
    }
}

impl Mul<Matrix3> for f64 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        Matrix3 {
            c1: rhs.c1 * self,
            c2: rhs.c2 * self,
            c3: rhs.c3 * self
        }
    }
}

impl MulAssign<f64> for Matrix3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.c1 *= rhs;
        self.c2 *= rhs;
        self.c3 *= rhs;
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return 
        write!(f, "[{:.4}, {:.4}, {:.4}]\n[{:.4}, {:.4}, {:.4}]\n[{:.4}, {:.4}, {:.4}]",
            self.c1.x, self.c2.x, self.c3.x, 
            self.c1.y, self.c2.y, self.c3.y,
            self.c1.z, self.c2.z, self.c3.z
        );
    }
}
