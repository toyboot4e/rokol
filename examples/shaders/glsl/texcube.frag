#version 330

uniform sampler2D tex;

in vec4 color;
in vec2 uv;

out vec4 outColor;

void main() {
    outColor = texture(tex, uv) * color;
}
