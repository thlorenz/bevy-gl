use bevy::{prelude::*, render::shader::ShaderStage};

use std::{env, error::Error, fs, str::from_utf8};

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
    materials.add(texture_handle.into())
}

pub fn write_to_tmp(
    feat_id: &str,
    filename: &str,
    content: String,
) -> Result<String, Box<dyn Error>> {
    let mut dir = env::temp_dir();
    dir.push("bevy-gl");
    dir.push(feat_id);
    fs::create_dir_all(dir.clone())?;

    let mut full_path = dir;
    full_path.push(filename);
    let full_path = full_path.to_str().unwrap();
    std::fs::write(full_path, content)?;
    return Ok(full_path.to_string());
}
