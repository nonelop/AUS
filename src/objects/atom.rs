use crate::geometry::shape::{GeometryShape, Shape};
use crate::algebra::vector::Vector3;
use crate::body::Body;

pub struct Atom {
    shape: Shape,
    pub velocity: Vector3,
    density: f64
}

impl Atom {
    pub fn new(shape: Shape) -> Self {
        Atom { shape, velocity: Vector3::ZERO, density: 1.0}
    }
}

impl Body for Atom {
    fn get_shape(&self) -> Shape {self.shape}
    fn set_shape(&mut self, shape: Shape) {self.shape = shape}

    fn get_velocity(&self) -> Vector3 {self.velocity}
    fn set_velocity(&mut self, velocity: Vector3) {self.velocity = velocity}

    fn get_mass(&self) -> f64 {
        self.density * self.get_shape().get_volume()
    }
}
