pub mod ops;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    pub const RIGHT: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0, z: 0.0 };

    pub const UP: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Self = Self { x: 0.0, y: -1.0, z: 0.0 };

    pub const FORWARD: Self = Self { x: 0.0, y: 0.0, z: -1.0 };
    pub const BACKWARD: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn normalize(&self) -> Self {
        let vector_length = self.length();

        if vector_length > 0.0 {
            let nx = self.x / vector_length;
            let ny = self.y / vector_length;
            let nz = self.z / vector_length;

            Vector3 { x: nx, y: ny, z: nz }
        }
        else {
            *self
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, b: Vector3) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(&self, b: Vector3) -> Vector3 {
        Vector3 { 
            x: self.y * b.z - self.z * b.y, 
            y: self.z * b.x - self.x * b.z, 
            z: self.x * b.y - self.y * b.x
        }
    }
}