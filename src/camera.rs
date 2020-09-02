use bevy::{
    input::{
        keyboard::ElementState,
        mouse::{MouseButtonInput, MouseMotion},
    },
    math::vec3,
    prelude::*,
    render::{
        camera::{Camera, PerspectiveProjection, VisibleEntities},
        render_graph::base,
    },
};
use std::ops::Mul;

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraState {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub zoom: f32,

    pub mov_speed: f32,
    pub mouse_sensitivity: f32,

    pub mouse_left_pressed: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        let mut camera = CameraState {
            position: vec3(0.0, 0.0, 3.0),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 0.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            world_up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            mov_speed: 0.005,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
            mouse_left_pressed: false,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl CameraState {
    pub fn get_view(&self) -> Mat4 {
        let target = self.position + self.front;
        Mat4::face_toward(self.position, target, self.up)
    }

    pub fn get_back_view(&self) -> Mat4 {
        let mut reversed = self.clone();
        reversed.yaw += 180.0_f32;
        reversed.pitch = -reversed.pitch;
        reversed.update_camera_vectors();
        reversed.get_view()
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, dt: u128) {
        let velocity = self.mov_speed * dt as f32;
        match direction {
            CameraMovement::Forward => {
                self.position += self.front.mul(velocity);
            }
            CameraMovement::Backward => {
                self.position -= self.front.mul(velocity);
            }
            CameraMovement::Left => {
                self.position -= self.right.mul(velocity);
            }
            CameraMovement::Right => {
                self.position += self.right.mul(velocity);
            }
        }
    }

    pub fn process_mouse_move(&mut self, dx: f32, dy: f32, constrain_pitch: bool) {
        let dx = dx * self.mouse_sensitivity;
        let dy = dy * self.mouse_sensitivity;

        self.yaw += dx;
        // Change this to += if not on a Mac, i.e. if your Y mouse/trackpad drag work backwards
        self.pitch -= dy;

        if constrain_pitch {
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

#[derive(Bundle)]
pub struct MyCamera {
    pub camera_state: CameraState,
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub translation: Translation,
    // pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for MyCamera {
    fn default() -> Self {
        Self {
            camera_state: Default::default(),
            camera: Camera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            // rotation: Default::default(),
            scale: Default::default(),
        }
    }
}

impl MyCamera {
    pub fn new(camera_state: CameraState) -> Self {
        let transform = Transform::new_sync_disabled(camera_state.get_view());
        MyCamera {
            camera_state,
            transform,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct MouseState {
    button_event_reader: EventReader<MouseButtonInput>,
    motion_event_reader: EventReader<MouseMotion>,
}

fn keyboard_motion_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut CameraState, &mut Transform)>,
) {
    let dt = time.delta.as_millis();
    for (mut camera_state, mut transform) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::W) {
            camera_state.process_keyboard(CameraMovement::Forward, dt);
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera_state.process_keyboard(CameraMovement::Backward, dt);
        }
        if keyboard_input.pressed(KeyCode::A) {
            camera_state.process_keyboard(CameraMovement::Left, dt);
        }
        if keyboard_input.pressed(KeyCode::D) {
            camera_state.process_keyboard(CameraMovement::Right, dt);
        }
        camera_state.update_camera_vectors();
        transform.value = camera_state.get_view();
    }
}

fn mouse_motion_system(
    mut mouse: ResMut<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&mut CameraState, &mut Transform)>,
) {
    for (mut camera_state, mut transform) in &mut query.iter() {
        for event in mouse.button_event_reader.iter(&mouse_button_input_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => camera_state.mouse_left_pressed = true,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                    ..
                } => camera_state.mouse_left_pressed = false,
                _ => {}
            };
        }

        // Only consider mouse motion events when the left mouse button is pressed
        if !camera_state.mouse_left_pressed {
            return;
        }

        for event in mouse.motion_event_reader.iter(&mouse_motion_events) {
            let delta: Vec2 = event.delta;
            camera_state.process_mouse_move(delta.x(), delta.y(), true);
        }
        camera_state.update_camera_vectors();
        transform.value = camera_state.get_view();
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseState>()
            .add_system(keyboard_motion_system.system())
            .add_system(mouse_motion_system.system());
    }
}
