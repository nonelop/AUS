use std::format;
use std::io::{self, Write};

use ecs::World;
use ecs_utils::*;
use env_logger;
use kiss3d::prelude::*;
use rand::seq::IndexedRandom;
use visual::Visual;
use wisk::prelude::*;
const DT: f64 = 0.01;

#[kiss3d::main]
async fn main() {
    env_logger::init();

    let mut visual = Visual::new();
    let mut window = Window::new("AUS Engine").await;
    let mut camera = OrbitCamera3d::default();
    visual.scene.add_light(Light::directional(Vec3 { x: 1.0, y: 0.0, z: 0.0 }));

    let mut world = World::new();

    let sphere1 = world.spawn();
    let sphere2 = world.spawn();

    world.add_component(
        sphere1.id,
        Position3D(Vector3 {
            x: -20.0,
            y: 0.0,
            z: 10.0,
        }),
    );
    world.add_component(
        sphere1.id,
        Velocity3D(Vector3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        }),
    );

    world.add_component(sphere1.id, Visulized {});
    world.add_component(sphere1.id, Sphere3D { radius: 1.0 });

    world.add_component(
        sphere2.id,
        Position3D(Vector3 {
            x: 20.0,
            y: 0.0,
            z: 10.0,
        }),
    );
    world.add_component(
        sphere2.id,
        Velocity3D(Vector3 {
            x: -5.0,
            y: 0.0,
            z: 0.0,
        }),
    );

    world.add_component(sphere2.id, Visulized {});
    world.add_component(sphere2.id, Sphere3D { radius: 1.0 });

    world.add_component(sphere1.id, Rotation3D(Quaternion::IDENTITY));
    world.add_component(sphere2.id, Rotation3D(Quaternion::IDENTITY));

    for _ in 0..=10 {
        world.spawn();
    }

    // Time test
    let mut i = 0;
    let mut times: Vec<std::time::Duration> = Vec::new();
    let mut rng = rand::rng();
    let types = ["Position3D", "Velocity3D", "Sphere3D"];
    let query_quantity = 100000;

    while i != query_quantity {
        let query: &str = &format!(
            "SELECT WITH {} OR WITHOUT {} WITH {}",
            types.choose(&mut rng).unwrap(),
            types.choose(&mut rng).unwrap(),
            types.choose(&mut rng).unwrap(),
        );
        let now = std::time::SystemTime::now();
        let _ = world.query(query).unwrap();
        let t = now.elapsed().unwrap();
        times.push(t);
        if i % 500 == 0 {
            print!("Time test progress: {:.2}% | Iteration time: {}\r", (i as f64 / query_quantity as f64) * 100.0, t.as_micros());
            io::stdout().flush().unwrap();
        }
        i += 1;
    }
    let mut total = 0;
    for t in times {
        total += t.as_micros();
    }
    println!(
        "TOTAL TIME {} QUERIES: {} | AVERAGE QUERY TIME: {}",
        query_quantity,
        total,
        total / query_quantity
    );

    while window.render_3d(&mut visual.scene, &mut camera).await {
        resolve_collision(&mut world);
        moving_system(&mut world);
        visual.update(&mut world);
    }
}

fn moving_system(world: &mut World) {
    let entities = world.query("SELECT WITH Position3D Velocity3D").unwrap();

    for e in &entities {
        let (position, velocity) = world
            .get_two_mut_components::<Position3D, Velocity3D>(e.id, e.id)
            .unwrap();

        position.0 += DT * velocity.0;
    }
}
