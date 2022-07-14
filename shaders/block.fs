#version 330 core

in vec2 v_tex_coords;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = vec4(texture(tex, v_tex_coords).rgb, 1.0);
}