use bevy::{
    math::vec2,
    prelude::*,
    render::{
        mesh::VertexAttribute,
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        shader::ShaderStages,
    },
};
use bevy_gl::libs::{app::app_default, util::vert_frag_shaders};

pub struct Triangle {
    /// Full width and height of the enclosing rectangle.
    pub size: Vec2,
}

impl From<Triangle> for Mesh {
    fn from(triangle: Triangle) -> Self {
        let extent_x = triangle.size.x() / 2.0;
        let extent_y = triangle.size.y() / 2.0;

        let north = vec2(0.0, extent_y / 2.0);
        let south_west = vec2(-extent_x, -extent_y);
        let south_east = vec2(extent_x, -extent_y);

        // Note that bevy doesn't allow us to pass color vertices, i.e. via Vertex_Color yet.
        // Instead we (ab)use the Normal vertex in order to pass this info needed by the frag
        // shader to interpolate.
        // Once this is possible we can change this example.
        // See bevy_render_src/mesh/mesh.rs:503
        // > TODO: allow pipelines to specialize on vertex_buffer_descriptor and index_format
        let vertices = [
            // bottom right
            (
                [south_east.x(), south_east.y(), 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0],
            ),
            // bottom left
            (
                [south_west.x(), south_west.y(), 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0],
            ),
            // top center
            ([north.x(), north.y(), 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ];

        let indices = vec![0, 2, 1];

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: Some(indices),
        }
    }
}

fn main() {
    app_default("Hello bevy Triangle".to_string())
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
        "src/basics/hello_triangle/shader.vert",
        "src/basics/hello_triangle/shader.frag",
    )
    .expect("Error loading shaders");

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(shader_vert),
        fragment: Some(shaders.add(shader_frag)),
    }));

    let render_pipeline = RenderPipeline::new(pipeline_handle);
    commands
        .spawn(MeshComponents {
            mesh: meshes.add(Mesh::from(Triangle {
                size: Vec2::new(2.0, 2.0),
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![render_pipeline]),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(0.0, 0.0, 3.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
