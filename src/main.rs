mod window;

use window::WindowTrait;

pub fn main() {
    let mut w = window::Window::new("SDL2 OpenGL Test", 800, 600);
    w.run();
}
