#version 330

uniform mat4 mvp;

layout(location=0) in vec4 in_pos;
layout(location=1) in vec4 in_color;
layout(location=2) in vec2 in_uv;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = mvp * in_pos;
    color = in_color;
    uv = in_uv;
}
