use bevy::prelude::*;

enum SpawnRequest {
    Cube,
    Sphere,
}

const CUBE_MESH: Handle<Mesh> = Handle::from_u128(9876876576541110);
const CUBE_MATERIAL: Handle<StandardMaterial> = Handle::from_u128(9876876576541111);
const SPHERE_MESH: Handle<Mesh> = Handle::from_u128(9876876576541112);
const SPHERE_MATERIAL: Handle<StandardMaterial> = Handle::from_u128(9876876576541113);

#[derive(Default)]
struct SpawnState {
    spawn_request: Option<SpawnRequest>,
    translation: Translation,
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(SpawnState::default())
            .add_startup_system(init_plugin.system())
            .add_system(keyboard_commands.system())
            .add_system(update_scene.system());
    }
}

fn init_plugin(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    meshes.set(CUBE_MESH, Mesh::from(shape::Cube { size: 1.0 }));
    materials.set(
        CUBE_MATERIAL,
        StandardMaterial {
            albedo: Color::rgb(0.5, 0.4, 0.3),
            ..Default::default()
        },
    );
    meshes.set(
        SPHERE_MESH,
        Mesh::from(shape::Icosphere {
            subdivisions: 4,
            radius: 1.0,
        }),
    );
    materials.set(
        SPHERE_MATERIAL,
        StandardMaterial {
            albedo: Color::rgb(0.5, 0.4, 0.3),
            ..Default::default()
        },
    );
}

fn keyboard_commands(mut state: ResMut<SpawnState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        state.spawn_request = Some(SpawnRequest::Cube);
    }
    if keyboard_input.just_pressed(KeyCode::Key2) {
        state.spawn_request = Some(SpawnRequest::Sphere);
    }
}

fn update_scene(mut commands: Commands, mut state: ResMut<SpawnState>) {
    match &state.spawn_request {
        Some(request) => {
            let (mesh, material) = match request {
                SpawnRequest::Cube => (CUBE_MESH, CUBE_MATERIAL),
                SpawnRequest::Sphere => (SPHERE_MESH, SPHERE_MATERIAL),
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
