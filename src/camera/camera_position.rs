use bevy::{math::vec3, prelude::*};

pub struct CameraPosition(pub Vec3);

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

impl From<Vec3> for CameraPosition {
    fn from(vec: Vec3) -> Self {
        CameraPosition(vec)
    }
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition(vec3(0.0, 0.0, 3.0))
    }
}
