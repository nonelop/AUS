// COMMENTED UNTIL ECS SYSTEMS DOESNT WORKS

// use crate::algebra::vector::Vector3;
// use crate::ecs::entity::Entity;
// use crate::physics::{CollisionInfo, CollisionResult, BodySnapshot};
// use crate::physics::collide::collide;

// pub fn resovle_collision(a: &mut Node, b: &mut Node) {
//     let snapshot_a = snapshot(a);
//     let snapshot_b = snapshot(b);

//     let info = CollisionInfo {
//         a: snapshot_a,
//         b: snapshot_b,

//         depth: 0.0,
//         normal: Vector3::ZERO,
//         contact_point: Vector3::ZERO
//     };
    
//     let result = collide(info);

//     apply_snapshot(result.a, a);
//     apply_snapshot(result.b, b);
// }

// fn snapshot(body: &mut Node) -> BodySnapshot {
//     BodySnapshot { 
//         position: body.position, 
//         velocity: body.velocity, 
//         angular_velocity: Vector3::ZERO, 
//         mass: 1.0
//     }
// }

// fn apply_snapshot(snapshot: BodySnapshot, body: &mut Node) {
//     body.position = snapshot.position;
//     body.velocity = snapshot.velocity;
// }