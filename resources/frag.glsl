#version 330 core
#extension GL_ARB_gpu_shader_fp64 : require
in vec2 uv;

out vec4 final_color;

uniform vec2 windowSize;

void main() {
    double x0 = double(gl_FragCoord.x)/windowSize.x;
    double y0 = double(gl_FragCoord.y)/windowSize.y;


    final_color = vec4(uv,0.0,1.0);
}