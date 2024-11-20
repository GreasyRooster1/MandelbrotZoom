#version 330 core
#extension GL_NV_gpu_shader_fp64 : require
in vec2 uv;

out vec4 final_color;

uniform vec2 windowSize;
uniform vec2 shift;
uniform float zoom;
uniform float maxIteration;
uniform float palRepeat;

uniform sampler2D palette_texture;

// Increment U


void main() {
    float64_t uvx= float64_t(uv.x)/100000;
    float64_t uvy= float64_t(uv.y)/100000;
    float64_t z = float64_t(zoom)/100000;
    float64_t shift_x = float64_t(shift.x)/100000;
    float64_t shift_y = float64_t(shift.y)/100000;
    float64_t x0 = float64_t((uvx*2.47 - 2.)/z + shift_x);
    float64_t y0 = float64_t((uvy*2.24 - 1.12)/z + shift_y);

    float64_t ix = float64_t(0.);
    float64_t iy = float64_t(0.);
    float64_t iteration = float64_t(0.);

    while(ix*ix + iy*iy <= 2.*2. && iteration < maxIteration){
        float64_t xtemp = ix*ix - iy*iy + x0;
        iy = 2.*ix*iy + y0;
        ix = xtemp;
        iteration = iteration + 1.;
    }

    float64_t ratio = float64_t(1.0-iteration/maxIteration);
    float64_t repeated_ratio = mod(ratio*palRepeat,1);

    final_color = texture(palette_texture,vec2(repeated_ratio,0.5));
}
