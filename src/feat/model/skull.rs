use bevy::prelude::*;
use bevy_gl::libs::{camera::camera_plugin::CameraTrait, util::load_texture_material};

// Source: https://sketchfab.com/3d-models/skull-downloadable-1a9db900738d44298b0bc59f68123393
fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_camera()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // TODO: how can we add normals?
    let base_material = load_texture_material(
        &asset_server,
        &mut textures,
        &mut materials,
        "resources/models/skull/textures/defaultMat_baseColor.png",
    );
    commands
        // mesh
        .spawn(PbrComponents {
            mesh: asset_server
                .load("resources/models/skull/scene.gltf")
                .unwrap(),
            material: base_material,
            rotation: Rotation::from_rotation_xyz(5.0, 0.5, 0.0),
            translation: Translation::new(0.0, 0.0, -1.0),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, 4.0),
            ..Default::default()
        });
}
