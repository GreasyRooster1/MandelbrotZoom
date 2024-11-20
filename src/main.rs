#![allow(clippy::single_match)]
use ogl33::*;
const WINDOW_TITLE: &str = "Hello Window";

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile},
    *,
};
use beryllium::video::GlSwapInterval;
use MandelbrotZoom::*;
use MandelbrotZoom::PolygonMode::{Fill, Line};

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] =
    [[1., 1., 0.0], [1., -1., 0.0], [-1., -1., 0.0], [-1., 1., 0.0]];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = include_str!("../resources/vert.glsl");

const FRAG_SHADER: &str = include_str!("../resources/frag.glsl");

fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    let mut flags = GlContextFlags::default();
    sdl.set_gl_context_flags(flags).unwrap();

    let win = sdl
        .create_gl_window(CreateWinArgs {
            title: WINDOW_TITLE,
            width: 800,
            height: 600,
            ..Default::default()
        })
        .expect("couldn't make a window and context");
    win.set_swap_interval(GlSwapInterval::Vsync).unwrap();

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    clear_color(0., 0., 0., 1.0);

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let shader_program =
        ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    let ebo = Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW,
    );

    let uni_size_loc = shader_program.register_uniform("windowSize");
    let uni_shift_loc = shader_program.register_uniform("shift");
    let uni_zoom_loc = shader_program.register_uniform("zoom");
    let uni_max_iteration_loc = shader_program.register_uniform("maxIteration");

    'main_loop: loop {
        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        // now the events are clear.


        unsafe{
            glClear(GL_COLOR_BUFFER_BIT);
            polygon_mode(Fill);
            glUniform2f(uni_size_loc, win.get_drawable_size().0 as GLfloat, win.get_drawable_size().1 as GLfloat);
            glUniform2f(uni_shift_loc, 0.0,0.0);
            glUniform1f(uni_zoom_loc, 1.);
            glUniform1f(uni_max_iteration_loc, 100.);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }
        win.swap_window();
        // here's where we could change the world state and draw.
    }
}