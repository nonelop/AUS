use ecs::{Query, World};
use wisk::vector::Vector3;

struct Position(Vector3);
struct Velocity(Vector3);

const DT: f64 = 0.001;

fn main() {
    let mut world = World::new();

    let entity = world.spawn();
    world.append_component(entity.id, Position(Vector3 { x: 0.0, y: 0.0, z: 0.0 }));
    world.append_component(entity.id, Velocity(Vector3 { x: 1.0, y: 0.0, z: 0.0 }));

    let mut counter = 0;

    loop {
        println!("[C]: {} [P]: {} [V]: {}",
        counter,
        world.get_component::<Position>(entity.id).unwrap().0,
        world.get_component::<Velocity>(entity.id).unwrap().0
        );
        moving_system(&mut world);

        counter += 1;
        std::thread::sleep(core::time::Duration::from_secs_f64(DT));
    }
}

fn moving_system(world: &mut World) {
    let entities = world.get_by_query(Query::new()
    .with::<Position>()
    .with::<Velocity>());

    for e in &entities {
        let (position, velocity) = world.get_two_mut_components::<Position, Velocity>(e.id, e.id).unwrap();

        position.0 += DT * velocity.0;
    }
}