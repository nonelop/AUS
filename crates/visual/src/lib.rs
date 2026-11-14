use std::collections::HashMap;

use ecs::{Entity, Query, World};
use ecs_utils::*;
use kiss3d::prelude::*;

pub struct Visual {
    pub scene: SceneNode3d,
    entity_node_map: HashMap<Entity, SceneNode3d>,
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            scene: SceneNode3d::empty(),
            entity_node_map: HashMap::new(),
        }
    }

    pub fn update(&mut self, world: &World) {
        let entities = world.get_by_query(
            Query::new()
                .with::<Visulized>()
                .with::<Position3D>()
                .with::<Rotation3D>(),
        );

        let mut to_delete: Vec<Entity> = Vec::new();

        for e in entities {
            match self.entity_node_map.get_mut(&e) {
                Some(node) => {
                    let pos = world.get_component::<Position3D>(e.id).unwrap();
                    let rot = world.get_component::<Rotation3D>(e.id).unwrap();

                    node.set_position(Vec3 {
                        x: pos.0.x as f32,
                        y: pos.0.y as f32,
                        z: pos.0.z as f32,
                    });
                    node.set_rotation(Quat::from_xyzw(
                        rot.0.x as f32,
                        rot.0.y as f32,
                        rot.0.z as f32,
                        rot.0.w as f32,
                    ));
                }
                None => {
                    if let Some(sphere) = world.get_component::<Sphere3D>(e.id) {
                        let node = self.scene.add_sphere(sphere.radius as f32);
                        self.entity_node_map.insert(e, node);
                    } else if let Some(cube) = world.get_component::<Cube3D>(e.id) {
                        let edge = cube.edge_size as f32;
                        let node = self.scene.add_cube(edge, edge, edge);
                        self.entity_node_map.insert(e, node);
                    }
                }
            }
        }

        for e in self.entity_node_map.keys() {
            if !world.check(e.id) {
                to_delete.push(*e);
            }
        }

        for i in to_delete {
            if let Some(mut node) = self.entity_node_map.remove(&i) {
                node.remove();
            }
        }
    }
}
