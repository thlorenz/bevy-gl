#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;

layout(location = 0) out vec3 v_Color;

layout(set = 0, binding = 0) uniform Camera {
    mat4 View;
};

void main() {
    gl_Position = View * vec4(Vertex_Position, 1.0);
    v_Color = Vertex_Normal;
}
