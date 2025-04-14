#![feature(never_type)]
mod game;
mod graphics;
mod math;
mod util;
mod window;
use game::game_window::GameWindow;
use log::info;
use window::WindowBase;

pub fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("log4rs configured!");

    let mut w = GameWindow::new("OpenGL Test", 800, 600);
    w.run();
    info!("Program closed!");
}
