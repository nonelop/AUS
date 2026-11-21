use super::{CollisionInfo, CollisionResult};

pub fn collide(info: &CollisionInfo) -> CollisionResult {
    let a = &info.a;
    let b = &info.b;

    let result = CollisionResult { a: *b, b: *a };

    result
}
