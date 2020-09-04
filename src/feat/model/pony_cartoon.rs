use bevy::{math::vec3, prelude::*};
use bevy_gl::{
    camera::{camera::Camera, camera_info::CameraInfoConfig, camera_plugin::CameraPlugin},
    util::load_texture_material,
};

// Source: https://sketchfab.com/3d-models/pony-cartoon-885d9f60b3a9429bb4077cfac5653cf9

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
    // TODO: how can we add normals and other textures like bottom and interior?
    let body_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/models/pony_cartoon/textures/Body_SG1_baseColor.png",
    );
    let car_mesh = asset_server
        .load("resources/models/pony_cartoon/scene.gltf")
        .unwrap();

    let car_scale: Scale = 0.02_f32.into();
    let car_rotation: Rotation = Rotation::from_rotation_xyz(-90.0_f32.to_radians(), 0.4, 0.0);
    let car_translation: Translation = Translation::new(0.0, -2.5, -12.0);

    commands
        // mesh
        .spawn(PbrComponents {
            mesh: car_mesh,
            scale: car_scale,
            rotation: car_rotation,
            translation: car_translation,
            material: body_material,
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
