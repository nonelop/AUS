use crate::geometry::shape::{GeometryShape, Shape}; 
use crate::geometry::vector::Vector3; 
pub trait Body {
    fn get_shape(&self) -> Shape;
    fn set_shape(&mut self, shape: Shape);

    fn get_velocity(&self) -> Vector3;
    fn set_velocity(&mut self, velocity: Vector3);

    fn get_mass(&self) -> f64; // Return mass value in kg

    fn get_position(&self) -> Vector3 {
        let shape = self.get_shape();

        match shape {
            Shape::Sphere(sphere) => sphere.get_center(),
            Shape::Cube(cube) => cube.get_center(),
        }
    }
    fn set_position(&mut self, position: Vector3) {
        let mut shape = self.get_shape();

        match &mut shape {
            Shape::Sphere(sphere) => {
                sphere.set_center(position);
                self.set_shape(Shape::Sphere(*sphere))
            }
            Shape::Cube(cube) => {  
                cube.set_center(position);
                self.set_shape(Shape::Cube(*cube))
            }
        }
    }
}
