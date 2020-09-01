use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        shader::ShaderStages,
    },
};
use bevy_gl::{app::app_default, util::vert_frag_shaders};

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
    app_default().add_startup_system(setup.system()).run();
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
