mod graphics;
use graphics::window::{Window, WindowTrait};

pub fn main() {
    let mut w = Window::new("SDL2 OpenGL Test", 800, 600);
    w.run();
}
