use bevy::prelude::*;
use bevy_gl::{
    feat::scene::SpawnPlugin,
    libs::{
        app::app_default,
        camera::{
            camera_info::CameraInfoConfig,
            camera_plugin::{AddCameraOpts, CameraTrait},
            camera_view::CameraViewOpts,
        },
    },
};

fn main() {
    app_default("Scene Save/Reload".to_string())
        .add_plugin(SpawnPlugin {})
        .add_startup_system(setup.system())
        .add_camera_from(AddCameraOpts {
            info: Some(CameraInfoConfig::default()),
            position: (12.24, 8.03, 11.26).into(),
            view: CameraViewOpts {
                pitch: -21.50,
                yaw: -136.00,
                ..Default::default()
            },
        })
        .run();

    eprintln!("Press '1' to spawn a Cube, '2' to spawn a Sphere");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_material = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: floor_material,
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        });
}
