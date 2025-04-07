use std::time::Duration;

use glfw::*;

pub struct Window {
    glfw: Glfw,
    window: PWindow,
    event_polls: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

pub trait WindowTrait {
    fn new(title: &str, width: u32, height: u32) -> Self;
    fn mainloop_logic(&mut self);
    fn event_logic(&mut self);
    fn run(&mut self);
}

impl WindowTrait for Window {
    fn mainloop_logic(&mut self) {
        unsafe {}
    }

    fn event_logic(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_polls) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                glfw::WindowEvent::Key(k, _, a, m) => {
                    println!("Key {:?} {:?} with mod {:?}", k, a, m);
                }
                _ => {}
            }
        }
    }

    fn run(&mut self) {
        while !self.window.should_close() {
            self.event_logic();
            self.mainloop_logic();

            // The rest of the game loop goes here...
            //

            self.window.swap_buffers();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Could not create window.");

        window.set_key_polling(true);
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        window.make_current();
        window.set_cursor_pos_polling(true);

        unsafe {
            gl::ClearColor(1., 0.3, 0.3, 0.0);
        }

        window.make_current();

        Self {
            glfw,
            window,
            event_polls: events,
        }
    }
}
