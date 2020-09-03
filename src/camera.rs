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

pub struct CameraPosition(Vec3);

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

pub struct CameraConfig {
    pub mov_speed: f32,
    pub mouse_sensitivity: f32,
    pub constrain_pitch: bool,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            mov_speed: 0.01,
            mouse_sensitivity: 0.1,
            constrain_pitch: true,
        }
    }
}

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

#[derive(Bundle)]
pub struct MyCamera {
    pub view: CameraView,
    pub position: CameraPosition,
    pub config: CameraConfig,
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
}

impl Default for MyCamera {
    fn default() -> Self {
        Self {
            view: Default::default(),
            position: Default::default(),
            config: Default::default(),
            camera: Camera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
        }
    }
}

#[derive(Default)]
struct MouseState {
    pub left_button_pressed: bool,
}

#[derive(Default)]
struct MouseEvents {
    button_events: EventReader<MouseButtonInput>,
    motion_events: EventReader<MouseMotion>,
}

fn keyboard_motion_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut CameraView,
        &mut Transform,
        &mut CameraPosition,
        &CameraConfig,
    )>,
) {
    let dt = time.delta.as_millis();
    for (mut camera_view, mut transform, mut position, config) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::W) {
            position.0 =
                camera_view.process_keyboard(CameraMovement::Forward, &position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::S) {
            position.0 =
                camera_view.process_keyboard(CameraMovement::Backward, &position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::A) {
            position.0 = camera_view.process_keyboard(CameraMovement::Left, &position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::D) {
            position.0 =
                camera_view.process_keyboard(CameraMovement::Right, &position, &config, dt);
        }
        transform.value = camera_view.get_view(&position);
    }
}

fn mouse_motion_system(
    mut mouse: ResMut<MouseEvents>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut mouse_state: ResMut<MouseState>,
    mut camera_query: Query<(
        &mut CameraView,
        &mut Transform,
        &CameraPosition,
        &CameraConfig,
    )>,
) {
    for (mut camera_view, mut transform, position, config) in &mut camera_query.iter() {
        for event in mouse.button_events.iter(&mouse_button_input_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => mouse_state.left_button_pressed = true,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                    ..
                } => mouse_state.left_button_pressed = false,
                _ => {}
            };
        }

        // Only consider mouse motion events when the left mouse button is pressed
        if !mouse_state.left_button_pressed {
            return;
        }

        for event in mouse.motion_events.iter(&mouse_motion_events) {
            let delta: Vec2 = event.delta;
            camera_view.process_mouse_move(delta.x(), delta.y(), &config);
        }
        camera_view.update_camera_vectors();
        transform.value = camera_view.get_view(&position);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseEvents>()
            .init_resource::<MouseState>()
            .add_system(keyboard_motion_system.system())
            .add_system(mouse_motion_system.system());
    }
}
