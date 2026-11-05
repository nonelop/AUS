use kiss3d::prelude::*;

use crate::geometry::shape::Shape;
use crate::math::vector::Vector3;
use crate::math::quaternion::Quaternion;

impl From<Vector3> for Vec3 {
    fn from(value: Vector3) -> Self {
        Self { x: value.x as f32, y: value.y as f32, z: value.z as f32 }
    }
}

impl From<Quaternion> for Quat {
    fn from(value: Quaternion) -> Self {
        Self::from_xyzw(
            value.x as f32, 
            value.y as f32, 
            value.z as f32, 
            value.w as f32
        )
    }
}

impl From<Shape> for SceneNode3d {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Sphere(sphere) => {
                SceneNode3d::sphere(sphere.get_radius() as f32)
            }
            Shape::Cube(cube) => {
                let edge = cube.get_edge() as f32;
                SceneNode3d::cube(edge, edge, edge)
            }
        }
    }
}