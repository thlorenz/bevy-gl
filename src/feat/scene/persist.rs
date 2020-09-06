use bevy::{prelude::*, type_registry::TypeRegistry};
use bevy_gl::libs::{
    app::app_default,
    camera::{
        camera_info::CameraInfoConfig,
        camera_plugin::{AddCameraOpts, CameraTrait},
        camera_view::CameraViewOpts,
    },
    util::write_to_tmp,
};

#[derive(Default, Debug)]
struct RequestState {
    pub requested_persist_scene: bool,
}

fn main() {
    app_default("Scene Save/Reload".to_string())
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::POST_UPDATE, persist_scene.thread_local_system())
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
        .add_resource(RequestState::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let material = materials.add(StandardMaterial {
        albedo: Color::rgb(0.5, 0.4, 0.3),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material,
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: cube_handle,
            material,
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        });
}

fn keyboard_commands(mut request_state: ResMut<RequestState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        request_state.requested_persist_scene = true;
    }
}

fn persist_scene(world: &mut World, resources: &mut Resources) {
    let mut request_state = resources.get_mut::<RequestState>().unwrap();
    if !request_state.requested_persist_scene {
        return;
    }

    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let scene = Scene::from_world(&world, &type_registry.component.read());

    let ron = scene
        .serialize_ron(&type_registry.property.read())
        .expect("Failed to serialize scene");

    let saved_to =
        write_to_tmp("persist_scene", "scene.ron", ron).expect("Failed to save serialized scene");
    println!("saved current scene to {}", saved_to);

    request_state.requested_persist_scene = false;
}
