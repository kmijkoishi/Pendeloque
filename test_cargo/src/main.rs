use test_cargo::graphics::window::Window;

fn main() {
    let mut window = Window::new(1080, 720, "hello world");

    window.init_gl();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT)
        }
        window.update();
    }
}
