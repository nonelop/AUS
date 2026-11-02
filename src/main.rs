mod math;
mod geometry;
mod physics;
mod body;
mod objects;
mod time;
mod errors;
mod config;
mod visual;
mod world;

use kiss3d::prelude::*;

use crate::geometry::shape::cube::Cube;
use crate::geometry::unit::GeometryUnits::Meter;
use crate::math::vec::Vector3;
use crate::geometry::shape::{Shape, sphere::Sphere};
use crate::objects::atom::Atom;
use crate::config::DELTA_TIME;
use crate::visual::Visual;
use crate::world::World;

#[kiss3d::main]
async fn main() {
    let mut world = World::new();
    let mut visual = Visual::new();

    let mut window = Window::new("SIMULATION").await;
    let mut camera = OrbitCamera3d::default();

    let atom = world.spawn(Box::new(Atom::new(
        Shape::Sphere(Sphere::new(Vector3 {x: 0.0, y: 0.25, z: 0.15}, Meter(1.0)).unwrap())
    )));

    let _atom2 = world.spawn(Box::new(Atom::new(
        Shape::Sphere(Sphere::new(Vector3 {x: 20.0, y: 0.0, z: 0.0}, Meter(1.0)).unwrap())
    )));

    let _cube = world.spawn(Box::new(Atom::new(
        Shape::Cube(Cube::new(Vector3 {x: 0.0, y: 2.0, z: 0.0}, Meter(1.0)).unwrap())
    )));

    let _cube2 = world.spawn(Box::new(Atom::new(
        Shape::Cube(Cube::new(Vector3 {x: 10.0, y: 2.0, z: 0.0}, Meter(1.0)).unwrap())
    )));

    world.get_mut_body(atom).unwrap().set_velocity(Vector3 {x: 10.0, y: 0.0, z: 0.0});

    while window.render_3d(&mut visual.scene, &mut camera).await {
        time::tick(DELTA_TIME, &mut world);
        visual.update(&world);
    }
}