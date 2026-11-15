use wisk::prelude::*;

pub mod collide;
#[derive(Clone, Copy)]
pub struct BodySnapshot {
    pub position: Vector3,

    pub velocity: Vector3,
    pub angular_velocity: Vector3,

    pub mass: f64,
}

#[derive(Clone, Copy)]
pub struct CollisionInfo {
    pub a: BodySnapshot,
    pub b: BodySnapshot,

    pub depth: f64,
    pub normal: Vector3,
    pub contact_point: Vector3,
}

#[derive(Clone, Copy)]
pub struct CollisionResult {
    pub a: BodySnapshot,
    pub b: BodySnapshot,
}
