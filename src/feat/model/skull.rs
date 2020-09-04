use bevy::{math::vec3, prelude::*};
use bevy_gl::{
    camera::{camera::Camera, camera_info::CameraInfoConfig, camera_plugin::CameraPlugin},
    util::load_texture_material,
};

// Source: https://sketchfab.com/3d-models/skull-downloadable-1a9db900738d44298b0bc59f68123393
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_plugin(CameraPlugin {
            camera_info: Some(CameraInfoConfig {
                interval_millis: 1000,
                ..Default::default()
            }),
            ..Default::default()
        })
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // TODO: how can we add normals?
    let base_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/models/skull/textures/defaultMat_baseColor.png",
    );
    commands
        // mesh
        .spawn(PbrComponents {
            mesh: asset_server
                .load("resources/models/skull/scene.gltf")
                .unwrap(),
            material: base_material,
            rotation: Rotation::from_rotation_xyz(5.0, 0.5, 0.0),
            translation: Translation::new(0.0, 0.0, -1.0),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn(Camera {
            position: vec3(0.0, 0.0, 3.0).into(),
            ..Default::default()
        });
}
