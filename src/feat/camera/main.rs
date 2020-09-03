use bevy::{math::vec3, prelude::*};
use bevy_gl::{
    app::app_default,
    camera::{CameraPlugin, MyCamera, CameraView},
};

fn main() {
    app_default("Hold left Mouse to move Camera".to_string())
        .add_startup_system(setup.system())
        .add_plugin(CameraPlugin)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(MyCamera {
            position: vec3(-3.0, 3.0, 8.0).into(),
            view: CameraView {
                pitch: -15.0,
                yaw: -100.0,
                ..Default::default()
            },
            ..Default::default()
        });
}
