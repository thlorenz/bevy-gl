use super::{
    camera_config::CameraConfig,
    camera_position::{CameraMovement, CameraPosition},
};
use bevy::{math::vec3, prelude::*};
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct CameraView {
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub zoom: f32,
}

impl Default for CameraView {
    fn default() -> Self {
        let mut camera = CameraView {
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            world_up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl CameraView {
    pub fn get_view(&self, position: &CameraPosition) -> Mat4 {
        let target = position.0 + self.front;
        Mat4::face_toward(position.0, target, self.up)
    }

    pub fn get_back_view(&self, position: &CameraPosition) -> Mat4 {
        let mut reversed = self.clone();
        reversed.yaw += 180.0_f32;
        reversed.pitch = -reversed.pitch;
        reversed.update_camera_vectors();
        reversed.get_view(position)
    }

    pub fn process_keyboard(
        &mut self,
        direction: CameraMovement,
        position: &CameraPosition,
        config: &CameraConfig,
        dt: u128,
    ) -> Vec3 {
        let velocity = config.mov_speed * dt as f32;
        match direction {
            CameraMovement::Forward => position.0 + self.front.mul(velocity),
            CameraMovement::Backward => position.0 - self.front.mul(velocity),
            CameraMovement::Left => position.0 - self.right.mul(velocity),
            CameraMovement::Right => position.0 + self.right.mul(velocity),
        }
    }

    pub fn process_mouse_move(&mut self, dx: f32, dy: f32, config: &CameraConfig) {
        let dx = dx * config.mouse_sensitivity;
        let dy = dy * config.mouse_sensitivity;

        self.yaw += dx;
        // Change this to += if not on a Mac, i.e. if your Y mouse/trackpad drag work backwards
        self.pitch -= dy;

        if config.constrain_pitch {
            self.constrain_pitch();
        }
        self.update_camera_vectors();
    }

    fn constrain_pitch(&mut self) {
        if self.pitch > 89.0 {
            self.pitch = 89.0
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0
        }
    }

    pub fn process_mouse_scroll(&mut self, dy: f32) {
        self.zoom -= dy;
        if self.zoom < 1.0 {
            self.zoom = 1.0
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }

    pub fn update_camera_vectors(&mut self) {
        let front = vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
