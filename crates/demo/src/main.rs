use std::format;

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

    let mut world = World::new();

    let entity = world.spawn();
    world.add_component(
        entity.id,
        Position3D(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        }),
    );
    world.add_component(
        entity.id,
        Velocity3D(Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    );

    world.add_component(
        entity.id,
        Rotation3D(Quaternion::from_axis(
            Vector3 {
                x: 0.5,
                y: 0.3,
                z: 0.2,
            },
            3.25,
        )),
    );
    world.add_component(entity.id, Visulized {});
    world.add_component(entity.id, Cube3D { edge_size: 1.0 });

    for _ in 0..=1000 {
        world.spawn();
    }

    // Time test
    let mut i = 0;
    let mut times: Vec<std::time::Duration> = Vec::new();
    let mut rng = rand::rng();
    let types = ["Position3D", "Velocity3D", "Cube3D"];
    let query_quantity = 100000;

    while i != query_quantity {
        println!("ITERATION: {}", i);
        let query: &str = &format!(
            "SELECT WITH {} OR WITHOUT {} WITH {}",
            types.choose(&mut rng).unwrap(),
            types.choose(&mut rng).unwrap(),
            types.choose(&mut rng).unwrap(),
        );
        let now = std::time::SystemTime::now();
        let _ = world.query(query).unwrap();
        times.push(now.elapsed().unwrap());
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
