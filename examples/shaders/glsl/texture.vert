#version 330

uniform vs_params {
    mat4 mvp;
};

layout(location=0) in vec4 pos0;
layout(location=1) in vec4 color0;
layout(location=2) in vec2 uv0;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = pos0;
    color = color0;
    uv = uv0;
}

