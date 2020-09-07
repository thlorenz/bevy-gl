use bevy::{prelude::*, type_registry::TypeRegistry};
use bevy_gl::{
    feat::scene::SpawnPlugin,
    libs::util::init_tmp_path,
    libs::util::save_to_tmp,
    libs::{
        app::app_default,
        camera::{
            camera_info::CameraInfoConfig,
            camera_plugin::{AddCameraOpts, CameraTrait},
            camera_view::CameraViewOpts,
        },
    },
};

enum Request {
    PersistScene,
    LoadScene,
}

#[derive(Default)]
struct RequestState {
    pub requested: Option<Request>,
}

fn main() {
    app_default("Scene Save/Reload".to_string())
        .add_startup_system(setup.system())
        .add_system(handle_persist_request.thread_local_system())
        .add_system(handle_load_request.system())
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
        .add_plugin(SpawnPlugin {})
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, -4.0),
            ..Default::default()
        });
}

fn keyboard_commands(mut request_state: ResMut<RequestState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        request_state.requested = Some(Request::PersistScene);
    }
    if keyboard_input.just_pressed(KeyCode::L) {
        request_state.requested = Some(Request::LoadScene);
    }
}

fn handle_persist_request(world: &mut World, resources: &mut Resources) {
    let mut request_state = resources.get_mut::<RequestState>().unwrap();

    match request_state.requested {
        Some(Request::PersistScene) => {
            let type_registry = resources.get::<TypeRegistry>().unwrap();
            let scene = Scene::from_world(&world, &type_registry.component.read());

            let ron = scene
                .serialize_ron(&type_registry.property.read())
                .expect("Failed to serialize scene");

            let saved_to = save_to_tmp("persist_scene", "scene.ron", ron)
                .expect("Failed to save serialized scene");
            println!("saved current scene to {}", saved_to);
            request_state.requested = None;
        }
        _ => {}
    }
}

fn handle_load_request(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut request_state: ResMut<RequestState>,
) {
    // This currently crashes, I tried loading the scene synchronously (via thread_local_system) as
    // well, but that crashes in the same manner with `MissingAssetHandler`.
    // This is the line that fails: `asset_server.load(scene_path.clone())`
    match request_state.requested {
        Some(Request::LoadScene) => {
            let scene_path = init_tmp_path("persist_scene", "scene.ron")
                .expect("Failed to load serialized scene, make sure to persist one first");

            let scene_handle: Handle<Scene> = asset_server
                .load(scene_path.clone())
                .expect("failed to load scene via assert server");
            scene_spawner.instance(scene_handle);
            scene_spawner.load(scene_handle);

            println!("loaded scene from {}", scene_path.clone());
            request_state.requested = None;
        }
        _ => {}
    }

    request_state.requested = None;
}
