pub mod cube;
pub mod sphere;

use cube::Cube;
use sphere::Sphere;
use crate::algebra::quaternion::Quaternion;
use crate::algebra::vector::Vector3;
use crate::physics::CollisionInfo;

// enum for group all shapes to one object
#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Cube(Cube),
    Sphere(Sphere)
}

// basic shape methods
pub trait GeometryShape {
    fn get_center(&self) -> Vector3;
    fn set_center(&mut self, center: Vector3);
    fn get_rotation(&self) -> Quaternion;
    fn set_rotation(&mut self, rotation: Quaternion);
    fn get_volume(&self) -> f64;
}

// methods for shape which can collide with other shapes
pub trait CollidableShape {
    fn is_colliding_with(&self, with: &Shape) -> bool;
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo>;
}

// wrapper which links to real implements
impl GeometryShape for Shape {
    fn get_center(&self) -> Vector3 {
        match self {
            Shape::Sphere(sphere) => sphere.get_center(),
            Shape::Cube(cube) => cube.get_center()
        }
    }

    fn set_center(&mut self, center: Vector3) {
        match self {
            Shape::Sphere(sphere) => sphere.set_center(center),
            Shape::Cube(cube) => cube.set_center(center),
        }
    }

    fn get_rotation(&self) -> Quaternion {
        match self {
            Shape::Sphere(sphere) => sphere.get_rotation(),
            Shape::Cube(cube) => cube.get_rotation(),
        }
    }

    fn set_rotation(&mut self, rotation: Quaternion) {
        match self {
            Shape::Sphere(sphere) => sphere.set_rotation(rotation),
            Shape::Cube(cube) => cube.set_rotation(rotation),
        }
    }

    fn get_volume(&self) -> f64 {
        match self {
            Shape::Sphere(sphere) => sphere.get_volume(),
            Shape::Cube(cube) => cube.get_volume(),
        }
    }
}

// wrapper which links to real implements
impl CollidableShape for Shape {
    fn is_colliding_with(&self, with: &Shape) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.is_colliding_with(with),
            Self::Cube(cube) => cube.is_colliding_with(with)
        }
    }
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo> {
        match self {
            Self::Sphere(sphere) => sphere.get_collision_info_with(with),
            Self::Cube(_cube) => None
        }
    }
}