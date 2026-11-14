use ecs::eql::resolver;
use ecs::{Query, World};
use ecs_utils::*;
use kiss3d::prelude::*;
use visual::Visual;
use wisk::prelude::*;

const DT: f64 = 0.01;

#[kiss3d::main]
async fn main() {
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

    let mut counter = 0;

    let entities = resolver("WITH (Rotation3D)", &world);
    match entities {
        Ok(entities) => {
            for e in entities { println!("{:?}", e.id) }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    while window.render_3d(&mut visual.scene, &mut camera).await {
        moving_system(&mut world);
        visual.update(&world);

        counter += 1;
    }
}

fn moving_system(world: &mut World) {
    let entities = world.get_by_query(Query::new().with::<Position3D>().with::<Velocity3D>());

    for e in &entities {
        let (position, velocity) = world
            .get_two_mut_components::<Position3D, Velocity3D>(e.id, e.id)
            .unwrap();

        position.0 += DT * velocity.0;
    }
}
