#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
#![allow(clippy::zero_ptr)]

use beryllium::*;
use test_cargo::learn_lib::*;
use gl33::*;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] =
    [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    void main()
    {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
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
    //let mut window = Window::new(1080, 720, "hello world");
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
        load_gl_with(|f_name| _win.get_proc_address(f_name.cast()));
        
        LearnLib::clear_color(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        ogl33::glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        ogl33::glBindVertexArray(vao);

        let mut vbo = 0;
        ogl33::glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        ogl33::glBindBuffer(ogl33::GL_ARRAY_BUFFER, vbo);
        ogl33::glBufferData(
            ogl33::GL_ARRAY_BUFFER,
            core::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            ogl33::GL_STATIC_DRAW,
        );

        ogl33::glVertexAttribPointer(
            0,
            3,
            ogl33::GL_FLOAT,
            ogl33::GL_FALSE,
            core::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        ogl33::glEnableVertexAttribArray(0);

        let vertex_shader = ogl33::glCreateShader(ogl33::GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        
        ogl33::glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        ogl33::glCompileShader(vertex_shader);
        let mut success = 0;
        ogl33::glGetShaderiv(vertex_shader, ogl33::GL_COMPILE_STATUS, &mut success);

        if success == 0
        {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            ogl33::glGetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = ogl33::glCreateShader(ogl33::GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        ogl33::glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        ogl33::glCompileShader(fragment_shader);
        
        let mut success = 0;
        ogl33::glGetShaderiv(fragment_shader, ogl33::GL_COMPILE_STATUS, &mut success);
        if success == 0
        {
            let mut v:  Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            ogl33::glGetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = ogl33::glCreateProgram();
        assert_ne!(shader_program, 0);
        ogl33::glAttachShader(shader_program, vertex_shader);
        ogl33::glAttachShader(shader_program, fragment_shader);
        ogl33::glLinkProgram(shader_program);

        let mut success = 0;
        ogl33::glGetProgramiv(shader_program, ogl33::GL_LINK_STATUS, &mut success);
        if success == 0
        {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            ogl33::glGetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        ogl33::glDeleteShader(vertex_shader);
        ogl33::glDeleteShader(fragment_shader);
        ogl33::glUseProgram(shader_program);
    }

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        unsafe {
            ogl33::glClear(ogl33::GL_COLOR_BUFFER_BIT);
            ogl33::glDrawArrays(ogl33::GL_TRIANGLES, 0, 3);
        }
        _win.swap_window();
    }
}
