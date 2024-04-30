//use test_cargo::graphics::window::Window;
// use beryllium::InitFlags;
// use beryllium::GlProfile;
// use beryllium::SDL;
use beryllium::*;


fn main() {
    //let mut window = Window::new(1080, 720, "hello world");
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
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
        .expect("couldn't");

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
    }

    // window.init_gl();

    // while !window.should_close() {
    //     unsafe {
    //         gl::ClearColor(0.3, 0.5, 0.3, 1.0);
    //         gl::Clear(gl::COLOR_BUFFER_BIT);
    //     }
    //     window.update();
    // }
}
