use bevy::prelude::*;
use bevy_gl::libs::{
    camera::{
        camera_plugin::{AddCameraOpts, CameraTrait},
        camera_view::CameraViewOpts,
    },
    util::load_texture_material,
};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_camera_from(AddCameraOpts {
            info: Some(Default::default()),
            position: (-2.55, 2.44, 5.51).into(),
            view: CameraViewOpts {
                pitch: -21.20,
                yaw: -72.40,
                ..Default::default()
            },
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
    let metal_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/textures/metal.png",
    );
    let planet_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/textures/planet.png",
    );
    commands
        // mesh
        .spawn(PbrComponents {
            // mesh loaded from gltf
            mesh: asset_server
                .load("resources/models/monkey/Monkey.gltf")
                .unwrap(),
            material: metal_material,
            translation: Translation::new(-1.5, 0.0, 0.0),
            ..Default::default()
        })
        // mesh loaded from glb
        .spawn(PbrComponents {
            mesh: asset_server
                .load("resources/models/monkey/Monkey.glb")
                .unwrap(),
            material: planet_material,
            translation: Translation::new(1.5, 0.0, 0.0),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, 4.0),
            ..Default::default()
        });
}
