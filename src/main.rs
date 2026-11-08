mod algebra;
mod geometry;
mod physics;
mod ecs;
mod time;
mod errors;
mod config;
mod visual;
mod bridge;

use kiss3d::prelude::*;

use crate::ecs::world::World;
use crate::geometry::shape::cube::Cube;
use crate::geometry::unit::GeometryUnits::Meter;
use crate::algebra::vector::Vector3;
use crate::algebra::quaternion::Quaternion;
use crate::geometry::shape::{Shape, sphere::Sphere};
use crate::config::DELTA_TIME;
// use crate::visual::Visual;

struct Position(Vector3);

#[kiss3d::main]
async fn main() {
    let mut world = World::new();
    // let mut visual = Visual::new();

    // let mut window = Window::new("SIMULATION").await;
    // let mut camera = OrbitCamera3d::default();

    let unit = world.spawn();

    world.insert(unit, Position);

//    while window.render_3d(&mut visual.scene, &mut camera).await {
        // time::tick(DELTA_TIME, &mut world);
        // visual.update(&world);
    // }
}
