use super::{camera_position::CameraPosition, camera_view::CameraView};
use bevy::prelude::*;

/// Note that this requires the CameraPlugin to be active
fn update_camera_info(
    dt: f32,
    info_config: &CameraInfoConfig,
    camera_view: &CameraView,
    camera_position: &CameraPosition,
) {
    let info = format!(
        "({:.2}, {:.2}, {:.2}) pitch: {:.2}, yaw: {:.2}, FPS: {:.0}",
        camera_position.pos().x(),
        camera_position.pos().y(),
        camera_position.pos().z(),
        camera_view.pitch,
        camera_view.yaw,
        (1000.0 / dt).round()
    );
    match info_config.output {
        CameraInfoOutput::WindowTitle => {
            eprintln!("Updating window title not supported yet.");
        }
        CameraInfoOutput::Console => {
            eprintln!("{}", info);
        }
    };
}

//
// A bug in bevy prevents this from working properly.
//
// If we change: mut camera_query: Query<(&CameraView, &CameraPosition)>,
//           to: mut camera_query: Query<(Changed<CameraView>, &CameraPosition)>,
// the following happens.
//
// 1. we enter the below function body no matter if CameraView was mutated or not
// 2. `&mut camera_query.iter()` never has any items (even if CameraView was mutated) and
//    thus we never enter the loop body
//
// Interestingly enough this works as expected inside the ./camera_plugin.rs where
// this plugin is initialized.
//
// For now we print camera info on each tick even if nothing about it changed.
//
fn on_camera_view_changed(
    info_config: Res<CameraInfoConfig>,
    mut info_state: ResMut<CameraInfoState>,
    mut camera_query: Query<(&CameraView, &CameraPosition)>,
) {
    if info_state.millis_since_last_update < info_config.interval_millis {
        return;
    }
    let dt = info_state.millis_since_last_update / info_state.frames_since_last_update;

    for (camera_view, camera_position) in &mut camera_query.iter() {
        update_camera_info(dt as f32, &info_config, camera_view, camera_position);
    }

    info_state.millis_since_last_update = 0;
    info_state.frames_since_last_update = 0;
}

/* Disabled for now as it only makes sense once @see on_camera_view_changed works
 * as expected.
fn on_camera_position_changed(
    info_config: Res<CameraInfoConfig>,
    mut info_state: ResMut<CameraInfoState>,
    mut camera_query: Query<(&CameraView, Changed<CameraPosition>)>,
) {
    if info_state.millis_since_last_update < info_config.interval_millis {
        return;
    }
    let dt = info_state.millis_since_last_update / info_state.frames_since_last_update;

    for (camera_view, camera_position) in &mut camera_query.iter() {
        update_camera_info(dt as f32, &info_config, camera_view, &*camera_position);
    }

    info_state.millis_since_last_update = 0;
    info_state.frames_since_last_update = 0;
}
 */

fn on_tick(time: Res<Time>, mut info_state: ResMut<CameraInfoState>) {
    let dt = time.delta.as_millis();
    info_state.millis_since_last_update += dt;
    info_state.frames_since_last_update += 1;
}

#[derive(Default, Debug)]
pub struct CameraInfoState {
    millis_since_last_update: u128,
    frames_since_last_update: u128,
}

#[derive(Clone, Copy)]
pub enum CameraInfoOutput {
    WindowTitle,
    Console,
}

#[derive(Clone, Copy)]
pub struct CameraInfoConfig {
    pub interval_millis: u128,
    pub output: CameraInfoOutput,
}

impl Default for CameraInfoConfig {
    fn default() -> Self {
        CameraInfoConfig {
            interval_millis: 1000,
            output: CameraInfoOutput::Console,
        }
    }
}

#[derive(Default)]
pub struct CameraInfoPlugin {
    pub config: CameraInfoConfig,
}

impl Plugin for CameraInfoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CameraInfoState>()
            .add_resource(self.config)
            .add_system(on_tick.system())
            .add_system(on_camera_view_changed.system());
    }
}
