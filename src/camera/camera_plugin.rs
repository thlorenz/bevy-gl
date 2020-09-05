use super::{
    camera_config::CameraConfig,
    camera_info::{CameraInfoConfig, CameraInfoPlugin},
    camera_position::{CameraMovement, CameraPosition},
    camera_view::CameraView,
};
use bevy::{
    input::{
        keyboard::ElementState,
        mouse::{MouseButtonInput, MouseMotion},
    },
    prelude::*,
};

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

// TODO: zoom
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
pub struct CameraPlugin {
    pub camera_info: Option<CameraInfoConfig>,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseEvents>()
            .init_resource::<MouseState>()
            .add_system(keyboard_motion_system.system())
            .add_system(mouse_button_system.system())
            .add_system(mouse_motion_system.system())
            .add_system(on_camera_view_changed.system())
            .add_system(on_camera_position_changed.system());

        match self.camera_info {
            Some(camera_info) => {
                app.add_plugin(CameraInfoPlugin {
                    config: camera_info,
                });
            }
            None => {}
        }
    }
}
