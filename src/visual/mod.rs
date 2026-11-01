use std::collections::HashMap;

use kiss3d::prelude::*;

use crate::geometry::shape::Shape;
use crate::geometry::vector::Vector3;
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
                        vector3_to_vec3(&world.get_body(i).unwrap().get_position())
                    );
                }
                None => {
                    let node = shape_to_node(&world.get_body(i).unwrap().get_shape()).unwrap();
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

pub fn vector3_to_vec3(v: &Vector3) -> Vec3 {
    Vec3 {
        x: v.x as f32,
        y: v.y as f32,
        z: v.z as f32,
    }
}

pub fn shape_to_node(shape: &Shape) -> Option<SceneNode3d> {
    match shape {
        Shape::Sphere(sphere) => {
            Some(SceneNode3d::sphere(sphere.get_radius() as f32))
        }
        Shape::Cube(cube) => {
            let edge = cube.get_edge() as f32;
            Some(SceneNode3d::cube(edge, edge, edge))
        }
    }
}