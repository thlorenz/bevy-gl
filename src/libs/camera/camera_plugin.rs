use super::{
    camera::Camera,
    camera_config::CameraConfig,
    camera_info::{CameraInfoConfig, CameraInfoPlugin},
    camera_position::{CameraMovement, CameraPosition},
    camera_view::{CameraView, CameraViewOpts},
};
use bevy::{
    input::{
        keyboard::ElementState,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
    render::camera::PerspectiveProjection,
};

#[derive(Default)]
struct MouseState {
    left_button_pressed: bool,
}

#[derive(Default)]
struct MouseEvents {
    button_events: EventReader<MouseButtonInput>,
    motion_events: EventReader<MouseMotion>,
    wheel_events: EventReader<MouseWheel>,
}

fn keyboard_motion_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&CameraView, &mut CameraPosition, &CameraConfig)>,
) {
    let dt = time.delta.as_millis();
    for (camera_view, mut position, config) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::W) {
            camera_view.process_keyboard(CameraMovement::Forward, &mut position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera_view.process_keyboard(CameraMovement::Backward, &mut position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::A) {
            camera_view.process_keyboard(CameraMovement::Left, &mut position, &config, dt);
        }
        if keyboard_input.pressed(KeyCode::D) {
            camera_view.process_keyboard(CameraMovement::Right, &mut position, &config, dt);
        }
    }
}

fn mouse_button_system(
    mut mouse: ResMut<MouseEvents>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mut mouse_state: ResMut<MouseState>,
) {
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
}

fn mouse_motion_system(
    mut mouse: ResMut<MouseEvents>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_state: Res<MouseState>,
    mut camera_query: Query<(&mut CameraView, &CameraConfig)>,
) {
    for (mut camera_view, config) in &mut camera_query.iter() {
        // Only consider mouse motion events when the left mouse button is pressed
        if !mouse_state.left_button_pressed {
            return;
        }

        for event in mouse.motion_events.iter(&mouse_motion_events) {
            let delta: Vec2 = event.delta;
            camera_view.process_mouse_move(delta.x(), delta.y(), &config);
        }
    }
}

fn mouse_wheel_system(
    mut mouse: ResMut<MouseEvents>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut camera_query: Query<(&mut CameraView, &mut PerspectiveProjection)>,
) {
    for (mut camera_view, mut projection) in &mut camera_query.iter() {
        for event in mouse.wheel_events.iter(&mouse_wheel_events) {
            let dy: f32 = event.y / 10.0;
            camera_view.process_mouse_wheel(dy);
            // TODO: not sure why this change doesn't affect the actual projection.
            // If we remove the projection from the Camera Entity then things don't
            // render properly, so projection is definitely taken into account, but
            // possibly only the very first time.
            projection.fov = camera_view.zoom.to_radians();
        }
    }
}

//
// Camera Mutation Handlers.
// Only one component can be Changed<> which is why we register a handler for either
// of the two camera components that require an update when they change.
//
fn on_camera_view_changed(
    mut camera_query: Query<(Changed<CameraView>, &CameraPosition, &mut Transform)>,
) {
    for (camera_view, position, mut transform) in &mut camera_query.iter() {
        update_camera(&camera_view, position, &mut transform);
    }
}

fn on_camera_position_changed(
    mut camera_query: Query<(&CameraView, Changed<CameraPosition>, &mut Transform)>,
) {
    for (camera_view, position, mut transform) in &mut camera_query.iter() {
        update_camera(camera_view, &position, &mut transform);
    }
}

fn update_camera(
    camera_view: &CameraView,
    position: &CameraPosition,
    mut transform: &mut Transform,
) {
    transform.value = camera_view.get_view(&position);
}

#[derive(Default)]
struct CameraPlugin {
    camera_info: Option<CameraInfoConfig>,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        match self.camera_info {
            Some(camera_info) => {
                app.add_plugin(CameraInfoPlugin {
                    config: camera_info,
                });
            }
            None => {}
        }
        app.init_resource::<MouseEvents>()
            .init_resource::<MouseState>()
            .add_system(keyboard_motion_system.system())
            .add_system(mouse_button_system.system())
            .add_system(mouse_motion_system.system())
            .add_system(mouse_wheel_system.system())
            .add_system(on_camera_view_changed.system())
            .add_system(on_camera_position_changed.system());
    }
}

//
// Add Camera Trait
//

pub struct AddCameraOpts {
    pub position: Vec3,
    pub view: CameraViewOpts,
    pub info: Option<CameraInfoConfig>,
}

impl Default for AddCameraOpts {
    fn default() -> Self {
        AddCameraOpts {
            position: CameraPosition::default().into(),
            view: Default::default(),
            info: None,
        }
    }
}

fn install_camera(mut commands: Commands, opts: Res<AddCameraOpts>) {
    commands.spawn(Camera {
        position: opts.position.into(),
        view: CameraView::new(&opts.view),
        ..Default::default()
    });
}

pub trait CameraTrait {
    fn add_camera_from(&mut self, opts: AddCameraOpts) -> &mut Self;
    fn add_camera(&mut self) -> &mut Self;
}

impl CameraTrait for AppBuilder {
    fn add_camera_from(&mut self, opts: AddCameraOpts) -> &mut Self {
        self.add_plugin(CameraPlugin {
            camera_info: opts.info,
        })
        .add_resource(opts)
        .add_startup_system(install_camera.system());
        self
    }
    fn add_camera(&mut self) -> &mut Self {
        self.add_camera_from(Default::default())
    }
}
