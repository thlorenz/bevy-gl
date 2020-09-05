use bevy::{math::vec3, prelude::*};

pub struct CameraPosition {
    pos: Vec3,
    pub is_dirty: bool,
}

impl CameraPosition {
    pub fn inc_pos(&mut self, delta: Vec3) {
        self.pos += delta;
        self.is_dirty = true;
    }

    pub fn dec_pos(&mut self, delta: Vec3) {
        self.pos -= delta;
        self.is_dirty = true;
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
        CameraPosition {
            pos,
            is_dirty: true,
        }
    }
}

impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition {
            pos: vec3(0.0, 0.0, 3.0),
            is_dirty: true,
        }
    }
}
