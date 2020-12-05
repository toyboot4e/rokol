#version 330

uniform mat4 mvp;

layout(location=0) in vec4 inPos;
layout(location=1) in vec4 inColor;
layout(location=2) in vec2 inUv;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = mvp * inPos;
    color = inColor;
    uv = inUv * 5.0;
}
