#version 330

uniform vs_params {
    mat4 mvp;
};

layout(location=0) in vec4 inPos;
layout(location=1) in vec4 inColor;
layout(location=2) in vec2 inUv;

out vec4 outColor;
out vec2 outUv;

void main() {
    gl_Position = mvp * inPos;
    outColor = inColor;
    outUv = inUv;
}

