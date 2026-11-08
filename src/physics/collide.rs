use crate::algebra::vector::Vector3;
use super::{CollisionInfo, CollisionResult, BodySnapshot};

pub fn collide(info: CollisionInfo) -> CollisionResult {
    let a = info.a;
    let b = info.b;

    let mut result = CollisionResult { a, b };

    let self_mass = a.mass;
    let with_mass = b.mass;

    let inv_self_mass = if self_mass <= 0.0 {0.0} else {1.0 / self_mass};
    let inv_with_mass = if with_mass <= 0.0 {0.0} else {1.0 / with_mass};
    let inv_total_mass = inv_self_mass + inv_with_mass;

    if inv_total_mass <= 0.0 {
        return result;
    }

    result.a.position = a.position + info.normal * (info.depth * inv_self_mass / inv_total_mass);
    result.a.position = b.position - info.normal * (info.depth * inv_with_mass / inv_total_mass);

    let self_velocity = a.velocity;
    let with_velocity = b.velocity;

    let v_rel = with_velocity - self_velocity;
    let v_rel_n = v_rel.dot(info.normal);

    if v_rel_n <= 0.0 {
        return result;
    }

    let restitution = 1.0;

    let j = (1.0 + restitution) * v_rel_n / inv_total_mass;

    result.a.velocity = self_velocity + info.normal * (j * inv_self_mass);
    result.b.velocity = with_velocity - info.normal * (j * inv_with_mass);

    result
}
