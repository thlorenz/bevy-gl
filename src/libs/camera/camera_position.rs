use bevy::{math::vec3, prelude::*};

#[derive(Debug)]
pub struct CameraPosition {
    pos: Vec3,
}

impl CameraPosition {
    pub fn inc_pos(&mut self, delta: Vec3) {
        self.pos += delta;
    }

    pub fn dec_pos(&mut self, delta: Vec3) {
        self.pos -= delta;
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

impl From<Vec3> for CameraPosition {
    fn from(pos: Vec3) -> Self {
        CameraPosition { pos }
    }
}

impl From<(f32, f32, f32)> for CameraPosition {
    fn from(pos: (f32, f32, f32)) -> Self {
        CameraPosition {
            pos: vec3(pos.0, pos.1, pos.2),
        }
    }
}

impl From<CameraPosition> for Vec3 {
    fn from(pos: CameraPosition) -> Self {
        pos.pos
    }
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition {
            pos: vec3(0.0, 0.0, 3.0),
        }
    }
}
