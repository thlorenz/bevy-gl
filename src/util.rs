use bevy::{prelude::*, render::shader::ShaderStage};

use std::{error::Error, str::from_utf8};

pub fn vert_frag_shaders(
    vertex_path: &str,
    frag_path: &str,
) -> Result<(Shader, Shader), Box<dyn Error>> {
    let vert_buffer = std::fs::read(&vertex_path)?;
    let frag_buffer = std::fs::read(&frag_path)?;
    Ok((
        Shader::from_glsl(ShaderStage::Vertex, from_utf8(&*vert_buffer)?),
        Shader::from_glsl(ShaderStage::Fragment, from_utf8(&*frag_buffer)?),
    ))
}

pub fn load_texture_material(
    asset_server: &AssetServer,
    mut textures: &mut Assets<Texture>,
    materials: &mut Assets<StandardMaterial>,
    path: &str,
) -> Handle<StandardMaterial> {
    let texture_handle = asset_server.load_sync(&mut textures, path).unwrap();

    let material = materials.add(StandardMaterial {
        albedo: Color::rgb(1.0, 1.0, 1.0),
        albedo_texture: Some(texture_handle),
        ..Default::default()
    });
    material
}
