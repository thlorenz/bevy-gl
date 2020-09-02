#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform Camera {
    mat4 View;
};

void main() {
    gl_Position = View * vec4(Vertex_Position, 1.0);
}
