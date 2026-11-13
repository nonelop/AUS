
use std::collections::HashMap;

use kiss3d::prelude::*;
use physics::shape::GeometryShape;
use ecs::{Entity, World, Query};
use std_components::*;

pub struct Visual {
    scene: SceneNode3d,
    body_node_map: HashMap<Entity, SceneNode3d>
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            scene: SceneNode3d::empty(),
            body_node_map: HashMap::new()
        }
    }

    pub fn update(&mut self, world: &World) {
        let entities = world.get_by_query(
            Query::new().with::<Visulized>().with::<Position3>().with::<Rotation3>()
        );

        let mut to_delete: Vec<Entity> = Vec::new();

        for e in entities {
            match self.body_node_map.get_mut(&e) {
                Some(node) => {
                    let pos = world.get_component::<Position3>(e.id).unwrap();
                    let rot = world.get_component::<Rotation3>(e.id).unwrap();

                    node.set_position(
                        Vec3 { x: pos.0.x as f32, y: pos.0.y as f32, z: pos.0.z as f32 }
                    );
                    node.set_rotation(Quat::from_xyzw(
                        rot.0.x as f32, rot.0.y as f32, rot.0.z as f32, rot.0.w as f32
                    ));

                }
                None => {

                }
            }
        }

        for i in self.body_node_map.keys() {
            // match world.g {
            //     Some(_body) => continue,
            //     None => to_delete.push(*i),
            // }
        }

        for i in to_delete {
            if let Some(mut node) = self.body_node_map.remove(&i) {
                node.remove();
            }
        }
    }
}