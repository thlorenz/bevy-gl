use super::{
    camera_config::CameraConfig, camera_position::CameraPosition, camera_view::CameraView,
};
use bevy::{
    prelude::*,
    render::{
        camera::{Camera as BevyCamera, PerspectiveProjection, VisibleEntities},
        render_graph::base,
    },
};

#[derive(Bundle)]
pub struct Camera {
    pub view: CameraView,
    pub position: CameraPosition,
    pub config: CameraConfig,
    pub camera: BevyCamera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        let view = CameraView::new(Default::default());
        Self {
            view,
            position: Default::default(),
            config: Default::default(),
            camera: BevyCamera {
                name: Some(base::camera::CAMERA3D.to_string()),
                ..Default::default()
            },
            perspective_projection: PerspectiveProjection {
                fov: view.zoom.to_radians(),
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Default::default(),
        }
    }
}
