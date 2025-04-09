mod graphics;
mod window;
use window::{Window, WindowTrait};
mod utility;
use log::{debug, error, info, log, trace, warn};

pub fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    debug!("log4rs configured!");

    let mut w = Window::new("SDL2 OpenGL Test", 800, 600);
    info!(
        "Window created with params: {0} {1}x{2}",
        w.init_params.0, w.init_params.1, w.init_params.2
    );
    w.run();
    info!("Program closed!");
}
