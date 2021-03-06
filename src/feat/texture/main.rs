use bevy::prelude::*;
use bevy_gl::libs::{
    app::app_default,
    camera::{
        camera_info::CameraInfoConfig,
        camera_plugin::{AddCameraOpts, CameraTrait},
        camera_view::CameraViewOpts,
    },
    util::load_texture_material,
};

fn main() {
    app_default("bevy texture".to_string())
        .add_startup_system(setup.system())
        .add_camera_from(AddCameraOpts {
            info: Some(CameraInfoConfig::default()),
            position: (6.40, 5.34, 7.17).into(),
            view: CameraViewOpts {
                pitch: -29.00,
                yaw: -135.00,
                ..Default::default()
            },
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let container_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/textures/container2.png",
    );

    let floor_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/textures/metal.png",
    );

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: floor_material,
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: container_material,
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            light: Light {
                color: Color::rgb(2.0, 2.0, 2.0),
                depth: 0.1..50.0,
                fov: f32::to_radians(60.0),
            },
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        });
}
