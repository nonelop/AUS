use wisk::prelude::*;

pub mod collide;

#[derive(Clone, Copy)]
struct KinematicProperties {
    position: Vector3,
    rotation: Quaternion,
    velocity: Vector3,
    angular_velocity: Vector3,
}

#[derive(Clone, Copy)]
struct PhysicsProperties {
    mass: f64
}

#[derive(Clone, Copy)]
pub struct Body {
    pub kinematic: KinematicProperties,
    pub physics: PhysicsProperties
}

pub struct CollisionInfo {
    pub a: Body,
    pub b: Body,

    pub depth: f64,
    pub normal: Vector3,
    pub contact_point: Vector3,
}

pub struct CollisionResult {
    pub a: Body,
    pub b: Body,
}
