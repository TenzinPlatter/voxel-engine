#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tex;

out vec2 tex_coord;

uniform mat4 proj;

void main() {
    // For 2D UI, convert vec2 position to vec4
    // z = 0.0 keeps it flat, w = 1.0 for proper matrix math
    gl_Position = proj * vec4(pos, 0.0, 1.0);
    tex_coord = tex;
}
