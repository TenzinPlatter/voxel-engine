#version 330 core
in vec2 tex_coord;
in vec3 our_color;

out vec4 frag_color;

uniform sampler2D our_texture;

void main() {
  frag_color = texture(our_texture, tex_coord);
}
