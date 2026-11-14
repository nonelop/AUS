use wisk::prelude::*;
use physics::{BodySnapshot, CollisionInfo, CollisionResult, collide::collide};
use ecs::{World, Query};

// == Most Used Components == 

// 3D Position components
pub struct Position3D(pub Vector3);
pub struct Velocity3D(pub Vector3);
pub struct Rotation3D(pub Quaternion);

// 3D Shape components
pub struct Sphere3D {
    pub radius: f64
}
pub struct Cube3D {
    pub edge_size: f64
}

// Marker components
pub struct Visulized {}

// == Bridge Systems ==

pub fn resolve_collision(world: &mut World) {
    let collidable = world.get_by_query(
        Query::new().with::<Position3D>().with::<Velocity3D>().with::<Rotation3D>()
    );

    for e in collidable {
    }
}