#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

use beryllium::*;
use test_cargo::learn_lib::*;
use gl33::*;
use ultraviolet::*;

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] =
    [[0.5, 0.5, 0.0], [0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0]];
const INDICES: [TriIndexes; 2] = [[0, 1, 2], [0, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
    uniform mat4 transform;

    layout (location = 0) in vec3 pos;

    void main()
    {
        //gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        gl_Position = transform * vec4(pos, 1.0);
    }
    "#;

const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;
    
    void main()
    {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
    "#;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    {
        sdl
            .set_gl_context_flags(video::GlContextFlags::FOWARD_COMPATIBLE)
            .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "test window",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let _win = sdl
        .create_gl_window(win_args)
        .expect("Couldn't make a window and context");

    _win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    unsafe {
        ogl33::load_gl_with(|f_name| _win.get_proc_address(f_name.cast()));
    }
    LearnLib::clear_color(0.2, 0.3, 0.3, 1.0);
    

    let vao = VertexArray::new().expect("Couldn't make VAO");
    vao.bind();

    let vbo = test_cargo::learn_lib::Buffer::new().expect("Couldn't make VBO");
    // let mut vbo = 0;
    // ogl33::glGenBuffers(1, &mut vbo);
    // assert_ne!(vbo, 0);

    vbo.bind(BufferType::Array);
    // ogl33::glBindBuffer(ogl33::GL_ARRAY_BUFFER, vbo);

    let ebo = test_cargo::learn_lib::Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        ogl33::GL_STATIC_DRAW,
    );

    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        ogl33::GL_STATIC_DRAW,
    );
    // ogl33::glBufferData(
    //     ogl33::GL_ARRAY_BUFFER,
    //     core::mem::size_of_val(&VERTICES) as isize,
    //     VERTICES.as_ptr().cast(),
    //     ogl33::GL_STATIC_DRAW,
    // );
    unsafe {
        ogl33::glVertexAttribPointer(
            0,
            3,
            ogl33::GL_FLOAT,
            ogl33::GL_FALSE,
            core::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        ogl33::glEnableVertexAttribArray(0);
    }

    let shader_program = 
        ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();


    polygon_mode(test_cargo::learn_lib::PolygonMode::Line);
    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        let time = sdl.get_ticks() as f32 / 1000.0_f32;
        let transform = Mat4::from_rotation_z(time);

        unsafe {
            ogl33::glClear(ogl33::GL_COLOR_BUFFER_BIT);
            let transform_name = test_cargo::null_str!("transform").as_ptr().cast();
            let transform_loc = 
                ogl33::glGetUniformLocation(shader_program.0, transform_name);
            ogl33::glUniformMatrix4fv(transform_loc, 1, ogl33::GL_FALSE, transform.as_ptr());
            ogl33::glDrawElements(ogl33::GL_TRIANGLES, 6, ogl33::GL_UNSIGNED_INT, 0 as *const _);
        }
        _win.swap_window();
    }
}
