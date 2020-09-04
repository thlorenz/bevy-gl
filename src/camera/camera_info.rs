use super::{camera_position::CameraPosition, camera_view::CameraView};
use bevy::prelude::*;

/// Note that this requires the CameraPlugin to be active
fn camera_info_system(
    time: Res<Time>,
    mut info_state: ResMut<CameraInfoState>,
    info_config: Res<CameraInfoConfig>,
    mut camera_query: Query<(&CameraView, &CameraPosition)>,
) {
    let dt = time.delta.as_millis();
    info_state.millis_since_last_update += dt;
    if info_state.millis_since_last_update < info_config.interval_millis {
        return;
    }

    for (camera_view, camera_position) in &mut camera_query.iter() {
        let info = format!(
            "({:.2}, {:.2}, {:.2}) pitch: {:.2}, yaw: {:.2}, FPS: {:.0}",
            camera_position.0.x(),
            camera_position.0.y(),
            camera_position.0.z(),
            camera_view.pitch,
            camera_view.yaw,
            (1000.0 / dt as f32).round()
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

    info_state.millis_since_last_update = 0;
}

#[derive(Default)]
pub struct CameraInfoState {
    millis_since_last_update: u128,
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
            .add_system(camera_info_system.system());
    }
}
