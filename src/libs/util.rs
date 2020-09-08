use bevy::{prelude::*, render::shader::ShaderStage};

use std::{env, error::Error, fs, io, str::from_utf8};

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

pub fn init_tmp_path(feat_id: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let mut dir = env::temp_dir();
    dir.push("bevy-gl");
    dir.push(feat_id);
    fs::create_dir_all(dir.clone())?;

    let mut full_path = dir;
    full_path.push(filename);
    let full_path = full_path.to_str().unwrap();
    Ok(full_path.to_string())
}

pub fn write_to(full_path: String, content: String) -> io::Result<()> {
    std::fs::write(full_path.clone(), content)
}

pub fn load_from(full_path: String) -> io::Result<String> {
    std::fs::read_to_string(full_path)
}

pub fn save_to_tmp(
    feat_id: &str,
    filename: &str,
    content: String,
) -> Result<String, Box<dyn Error>> {
    let full_path = init_tmp_path(feat_id, filename)?;
    write_to(full_path.clone(), content)?;
    Ok(full_path)
}

pub fn load_from_tmp(feat_id: &str, filename: &str) -> Result<(String, String), Box<dyn Error>> {
    let full_path = init_tmp_path(feat_id, filename)?;
    let content = load_from(full_path.clone())?;
    Ok((full_path, content))
}
