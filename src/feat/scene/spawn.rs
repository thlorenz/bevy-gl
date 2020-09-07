use bevy::prelude::*;
use bevy_gl::libs::{
    app::app_default,
    camera::{
        camera_info::CameraInfoConfig,
        camera_plugin::{AddCameraOpts, CameraTrait},
        camera_view::CameraViewOpts,
    },
};

enum SpawnRequest {
    Cube,
    Sphere,
}

#[derive(Default)]
struct State {
    pub spawn_request: Option<SpawnRequest>,
    pub cube_mesh: Handle<Mesh>,
    pub cube_material: Handle<StandardMaterial>,
    pub sphere_mesh: Handle<Mesh>,
    pub sphere_material: Handle<StandardMaterial>,
    pub translation: Translation,
}

fn main() {
    app_default("Scene Save/Reload".to_string())
        .add_resource(State::default())
        .add_startup_system(setup.system())
        .add_system(update_scene.system())
        .add_camera_from(AddCameraOpts {
            info: Some(CameraInfoConfig::default()),
            position: (12.24, 8.03, 11.26).into(),
            view: CameraViewOpts {
                pitch: -21.50,
                yaw: -136.00,
                ..Default::default()
            },
        })
        .add_system(keyboard_commands.system())
        .run();

    println!("Press '1' to spawn a Cube, '2' to spawn a Sphere");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<State>,
) {
    let floor_material = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });

    state.cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    state.cube_material = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });
    state.sphere_mesh = meshes.add(Mesh::from(shape::Icosphere {
        subdivisions: 4,
        radius: 1.0,
    }));
    state.sphere_material = materials.add(StandardMaterial {
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

fn keyboard_commands(mut state: ResMut<State>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        state.spawn_request = Some(SpawnRequest::Cube);
    }
    if keyboard_input.just_pressed(KeyCode::Key2) {
        state.spawn_request = Some(SpawnRequest::Sphere);
    }
}

fn update_scene(mut commands: Commands, mut state: ResMut<State>) {
    match &state.spawn_request {
        Some(request) => {
            let (mesh, material) = match request {
                SpawnRequest::Cube => (state.cube_mesh, state.cube_material),
                SpawnRequest::Sphere => (state.sphere_mesh, state.sphere_material),
            };

            state.translation.0 += Vec3::new(0.0, 2.0, 0.0);

            commands.spawn(PbrComponents {
                mesh,
                material,
                translation: state.translation,
                ..Default::default()
            });

            state.spawn_request = None;
        }
        None => {}
    }
}
