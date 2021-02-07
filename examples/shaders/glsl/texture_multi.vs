#version 330

uniform vs_params {
    mat4 mvp;
};

layout(location=0) in vec3 in_pos;
layout(location=1) in vec4 in_color;
layout(location=2) in vec2 in_uv;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = vec4(in_pos, 1.0);
    color = in_color;
    uv = in_uv;
}
