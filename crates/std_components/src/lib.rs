use wisk::prelude::*;

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