use crate::algebra::vector::Vector3;
use crate::body::Body;
use crate::physics::{CollisionInfo, CollisionResult, BodySnapshot};
use crate::physics::collide::collide;
use crate::world::{self, BodyId};

pub fn resovle_collision(a: &mut dyn Body, b: &mut dyn Body) {
    let snapshot_a = snapshot(a);
    let snapshot_b = snapshot(b);

    let info = CollisionInfo {
        a: snapshot_a,
        b: snapshot_b,

        depth: 0.0,
        normal: Vector3::ZERO,
        contact_point: Vector3::ZERO
    };
    
    let result = collide(info);

    apply_snapshot(result.a, a);
    apply_snapshot(result.b, b);
}

fn snapshot(body: &mut dyn Body) -> BodySnapshot {
    BodySnapshot { 
        position: body.get_position(), 
        velocity: body.get_velocity(), 
        angular_velocity: Vector3::ZERO, 
        mass: body.get_mass()
    }
}

fn apply_snapshot(snapshot: BodySnapshot, body: &mut dyn Body) {
    body.set_position(snapshot.position);
    body.set_velocity(snapshot.velocity);
}