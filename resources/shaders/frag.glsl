#version 330 core
#extension GL_ARB_gpu_shader_fp64 : require
in vec2 uv;

out vec4 final_color;

uniform vec2 windowSize;
uniform vec2 shift;
uniform float zoom;
uniform float maxIteration;
uniform float palRepeat;

uniform sampler2D palette_texture;

void main() {
    float x0 = (uv.x*2.47 - 2.)/zoom + shift.x;
    float y0 = (uv.y*2.24 - 1.12)/zoom + shift.y;

    float ix = 0.;
    float iy = 0.;
    float iteration = 0.;

    while(ix*ix + iy*iy <= 2.*2. && iteration < maxIteration){
        float xtemp = ix*ix - iy*iy + x0;
        iy = 2.*ix*iy + y0;
        ix = xtemp;
        iteration = iteration + 1.;
    }

    float ratio = 1.0-iteration/maxIteration;
    float repeated_ratio = mod(ratio*palRepeat,1);

    final_color = texture(palette_texture,vec2(repeated_ratio,0.5));
}