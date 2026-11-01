use crate::errors::GeometryErrors;
use crate::geometry::shape::GeometryShape;
use crate::geometry::vector::Vector3;
use crate::geometry::unit::GeometryUnits;
use crate::physics::collide::CollisionInfo;
use super::CollidableShape;
use super::Shape;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: GeometryUnits) -> Result<Self, GeometryErrors> {
        let radius = radius.to_base_unit().get_value_in_base();
        
        if radius <= 0.0 {
            return Err(GeometryErrors::InvalidSizeValue(
                radius,
            ));
        }

        Ok(Sphere {
            center: position,
            radius: radius,
        })
    }
    pub fn get_radius(&self) -> f64 { self.radius }
}

impl GeometryShape for Sphere {
    fn get_center(&self) -> Vector3 { self.center }
    fn set_center(&mut self, center: Vector3) {
        self.center = center;
    }
    fn get_volume(&self) -> f64 {
        (4.0/3.0) * PI * self.radius.powi(3)
    }
}

impl CollidableShape for Sphere {
    fn is_colliding_with(&self, with: &Shape) -> bool {
        match with {
            Shape::Sphere(sphere) => {
                let d = (sphere.center.x - self.center.x).powi(2) + (sphere.center.y - self.center.y).powi(2) + (sphere.center.z - self.center.z).powi(2);
                if d <= (self.radius + sphere.radius).powi(2) {
                    return true;
                }
                else {
                    return false;
                }
            }
            Shape::Cube(_cube) => {
                return false; // pass
            }
        }
    }
    fn get_collision_info_with(&self, with: &Shape) -> Option<CollisionInfo> {
        if self.is_colliding_with(with) {
            match with {
                Shape::Sphere(sphere) => {
                    let depth = (self.radius + sphere.radius) - (self.center - sphere.center).length();
                    let normal = (self.center - sphere.center).normalize();
                    let contact_point = self.center + (self.radius * normal);

                    Some(CollisionInfo {
                        depth,
                        normal,
                        contact_point
                    })
                }
                Shape::Cube(_cube) => {
                    None
                }
            }
        }
        else {
            None
        }
    }
}