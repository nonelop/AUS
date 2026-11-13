use ecs::{Query, World};
use wisk::prelude::*;
use std_components::*;
use kiss3d::prelude::*;
use visual::Visual;

const DT: f64 = 0.01;

#[kiss3d::main]
async fn main() {
    let mut visual = Visual::new();
    let mut window = Window::new("Kiss3d: cube").await;
    let mut camera = OrbitCamera3d::default();

    let mut world = World::new();

    let entity = world.spawn();
    world.add_component(entity.id, Position3D(Vector3 { x: 0.0, y: 0.0, z: 10.0 }));
    world.add_component(entity.id, Velocity3D(Vector3 { x: 10.0, y: 0.0, z: 0.0 }));
    world.add_component(entity.id, Rotation3D(Quaternion::IDENTITY));
    world.add_component(entity.id, Visulized {});
    world.add_component(entity.id, Sphere3D { radius: 2.5 });

    let mut counter = 0;

    while window.render_3d(&mut visual.scene, &mut camera).await {
        println!("[C]: {} [P]: {} [V]: {}",
        counter,
        world.get_component::<Position3D>(entity.id).unwrap().0,
        world.get_component::<Velocity3D>(entity.id).unwrap().0
        );

        moving_system(&mut world);
        visual.update(&world);

        counter += 1;
    }
}

fn moving_system(world: &mut World) {
    let entities = world.get_by_query(Query::new()
    .with::<Position3D>()
    .with::<Velocity3D>());

    for e in &entities {
        let (position, velocity) = world.get_two_mut_components::<Position3D, Velocity3D>(e.id, e.id).unwrap();

        position.0 += DT * velocity.0;
    }
}
