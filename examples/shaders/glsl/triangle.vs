#version 330

layout(location=0) in vec4 in_pos;
layout(location=1) in vec4 in_color;

out vec4 color;

void main() {
    gl_Position = in_pos;
    color = in_color;
}
