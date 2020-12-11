#version 330

uniform sampler2D tex1;
uniform sampler2D tex2;

in vec3 color;
in vec2 uv;

out vec4 frag_color;

void main() {
    /* tex1 : tex2 = 2 : 8 */
    frag_color = mix(texture(tex1, uv), texture(tex2, uv), 0.2);
}
