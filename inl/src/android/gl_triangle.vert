#version 300 es

uniform mat4 projection;
in vec2 position;

void main() {
    gl_Position = projection * vec4(position, 0.0, 1.0);
}
