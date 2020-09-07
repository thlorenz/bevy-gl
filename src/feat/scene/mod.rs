use bevy::prelude::*;

enum SpawnRequest {
    Cube,
    Sphere,
}

#[derive(Default)]
struct SpawnState {
    pub spawn_request: Option<SpawnRequest>,
    pub cube_mesh: Handle<Mesh>,
    pub cube_material: Handle<StandardMaterial>,
    pub sphere_mesh: Handle<Mesh>,
    pub sphere_material: Handle<StandardMaterial>,
    pub translation: Translation,
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

fn init_plugin(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<SpawnState>,
) {
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
