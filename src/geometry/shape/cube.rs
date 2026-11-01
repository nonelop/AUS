use crate::errors::GeometryErrors;
use crate::geometry::shape::{CollidableShape, GeometryShape, Shape};
use crate::geometry::unit::GeometryUnits;
use crate::geometry::vector::Vector3;
use crate::physics::collide::CollisionInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    center: Vector3,
    edge: f64
}

impl Cube {
    pub fn new(position: Vector3, edge: GeometryUnits) -> Result<Self, GeometryErrors> {
        if edge.get_value_in_base() < 0.0 {
            return Err(GeometryErrors::InvalidSizeValue(edge.get_value_in_base()));
        }
        Ok(Cube { center: position, edge: edge.get_value_in_base() })
    }

    pub fn get_edge(&self) -> f64 { self.edge }
}

impl GeometryShape for Cube {
    fn get_center(&self) -> Vector3 {
        self.center
    }
    fn set_center(&mut self, center: Vector3) {
        self.center = center
    }
    fn get_volume(&self) -> f64 {
        self.edge.powi(3)
    }
}

impl CollidableShape for Cube {
    fn is_colliding_with(&self, with: &Shape) -> bool {
        match with {
            Shape::Sphere(_sphere) => {
                false
            }
            Shape::Cube(cube) => {
                let radius = (self.get_edge() + cube.get_edge()) * 0.5;

                let dx = (self.get_center().x - cube.get_center().x).abs();
                let dy = (self.get_center().y - cube.get_center().y).abs();
                let dz = (self.get_center().z - cube.get_center().z).abs();

                dx <= radius && dy <= radius && dz <= radius
            }
        }
    }
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo> {
        if self.is_colliding_with(with) {
            match with {
                Shape::Sphere(_sphere) => None,
                Shape::Cube(_cube) => {
                    Some(CollisionInfo {
                        depth: 0.0,
                        normal: Vector3::zero(),
                        contact_point: Vector3::zero()
                    })
                }
            }
        }
        else {
            None
        }
    }
}