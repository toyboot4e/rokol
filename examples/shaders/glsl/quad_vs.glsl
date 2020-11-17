#version 330
in vec4 position;
in vec4 color0;
out vec4 color;

void main() {
    gl_Position = position;
    color = color0;
}
