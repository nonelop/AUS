use crate::geometry::shape::CollidableShape;
use crate::world::World;


pub fn tick(dt: f64, world: &mut World) {

    let mut objects = world.get_body_ids();

    for obj in &mut objects {
        let obj = world.get_mut_body(*obj).unwrap();
        obj.move_with_velocity(dt);
    }

    for obj in objects.iter() {
        for with in objects.iter() {
            if obj != with {            
                let (obj_body, with_body) = world.get_many_mut_body(*obj, *with).unwrap();

                if obj_body.get_shape().is_colliding_with(
                    &with_body.get_shape()
                ) {
                    let info = obj_body
                    .get_shape()
                    .get_collision_info_with(&with_body.get_shape())
                    .unwrap();

                    obj_body.collide(
                        with_body,
                        info
                    );
                }
            }
        }
    }
}
