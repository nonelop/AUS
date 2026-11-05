pub mod comp;

use std::collections::HashMap;

use kiss3d::prelude::*;

use crate::geometry::shape::Shape;
use crate::geometry::shape::GeometryShape;
use crate::math::vector::Vector3;
use crate::world::{BodyId, World};

pub struct Visual {
    pub scene: SceneNode3d,
    body_node_map: HashMap<BodyId, SceneNode3d>
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            scene: SceneNode3d::empty(),
            body_node_map: HashMap::new()
        }
    }

    pub fn update(&mut self, world: &World) {
        let body_ids = world.get_body_ids();
        let mut to_delete: Vec<BodyId> = Vec::new();

        for i in body_ids {
            match self.body_node_map.get_mut(&i) {
                Some(node) => {
                    node.set_position(
                        Vec3::from(world.get_body(i).unwrap().get_position())
                    );
                    node.set_rotation(
                        Quat::from(world.get_body(i).unwrap().get_shape().get_rotation())
                    );
                }
                None => {
                    let node = SceneNode3d::from(world.get_body(i).unwrap().get_shape());
                    self.scene.add_child(node.clone());
                    self.body_node_map.insert(i, node);
                }
            }
        }

        for i in self.body_node_map.keys() {
            match world.get_body(*i) {
                Some(_body) => continue,
                None => to_delete.push(*i),
            }
        }

        for i in to_delete {
            if let Some(mut node) = self.body_node_map.remove(&i) {
                node.remove();
            }
        }
    }
}