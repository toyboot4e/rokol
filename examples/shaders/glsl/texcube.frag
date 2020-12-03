#version 330

uniform sampler2D tex;

in vec4 inColor;
in vec2 inUv;
out vec4 outColor;

void main() {
    outColor = texture(tex, inUv) * inColor;
}
