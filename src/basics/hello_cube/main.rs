use bevy::{
    prelude::*,
    render::{
        pass::ClearColor,
        pipeline::{PipelineDescriptor, RenderPipeline},
        shader::{ShaderStage, ShaderStages},
    },
    window::WindowMode,
};
use std::{error::Error, str::from_utf8};

fn vert_frag_shaders(
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

/**
 * This is the most basic example using shaders that I could come up with.
 *
 * Note that we don't use  PipelineSpecialization like in bevy examples as we need
 * no transform nor material.
 * Our color is hardcoded in the frag shader instead.
 *
 * We do need a camera as otherwise we don't see the cube at all.
 */
fn main() {
    let window_config: WindowDescriptor = WindowDescriptor {
        title: "Hello Cube!".to_string(),
        width: 1600,
        height: 1200,
        vsync: true,
        resizable: false,
        mode: WindowMode::Windowed,
        ..Default::default()
    };

    let anti_alias_config: Msaa = Msaa { samples: 4 };
    let clear_background: ClearColor = ClearColor(Color::rgb(0.02, 0.03, 0.03));

    App::build()
        .add_resource(anti_alias_config)
        .add_resource(window_config)
        .add_resource(clear_background)
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (shader_vert, shader_frag) = vert_frag_shaders(
        "src/basics/hello_cube/shader.vert",
        "src/basics/hello_cube/shader.frag",
    )
    .expect("Error loading shaders");

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(shader_vert),
        fragment: Some(shaders.add(shader_frag)),
    }));

    let render_pipeline = RenderPipeline::new(pipeline_handle);
    commands
        .spawn(MeshComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![render_pipeline]),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(-3.0, 3.0, 5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
