#version 330 core
layout (location = 0) in vec3 pos;

out vec4 vertex_color;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    vertex_color = vec4(pos.x/2+0.5, 1-(pos.y/2+0.5), pos.z, 1.0);
}