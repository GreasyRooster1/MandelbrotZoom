#version 330 core
layout (location = 0) in vec3 pos;

out vec2 uv;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    uv = vec2((pos.x/2+0.5)*100000, (1-(pos.y/2+0.5))*100000);
}