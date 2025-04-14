use glfw::*;
use log::info;

pub struct Window {
    pub glfw: Glfw,
    pub window: PWindow,
    pub event_polls: glfw::GlfwReceiver<(f64, WindowEvent)>,
    pub init_params: (String, u32, u32),
}

/// The base trait for the window.
pub trait WindowBase {
    /// Initializes the window with the given title and dimensions.
    ///
    /// **THIS SHOULD NEVER CREATE/STORE SHADERS**
    ///
    /// Shaders should be created/stored during the `WindowBase::pre_load` function.
    fn new(title: &str, width: u32, height: u32) -> Self;
    /// Called every frame to update the window.
    ///
    /// **THIS SHOULD NOT DEAL WITH THE RENDERING OF SHADERS**
    ///
    /// Shader rendering is done on the `WindowBase::on_render_frame` function.
    fn on_update_frame(&mut self) {
        crate::util::gl::funcs::check_gl_error("WindowBase::on_update_frame");
    }
    /// Called every frame to render the shaders.
    fn on_render_frame(&mut self) {
        crate::util::gl::funcs::check_gl_error("WindowBase::on_render_frame");
    }
    /// Called every frame to handle events.
    ///
    /// Events included are:
    /// - Key Presses
    /// - Mouse Movement
    /// - Window Resizing
    /// - Window Closing
    fn event_logic(&mut self) {
        crate::util::gl::funcs::check_gl_error("WindowBase::event_logic");
    }
    /// Called before the window is loaded.
    fn pre_load(&mut self) {
        crate::util::gl::funcs::check_gl_error("WindowBase::pre_load");
    }
    /// Called when the window is unloaded.
    fn unload(&mut self) {
        crate::util::gl::funcs::check_gl_error("WindowBase::unload");
    }
    /// Called to run the window.
    fn run(&mut self) {}
}

impl WindowBase for Window {
    fn on_update_frame(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }
    fn event_logic(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_polls) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    fn pre_load(&mut self) {}

    fn run(&mut self) {
        while !self.window.should_close() {
            self.event_logic();
            self.on_update_frame();
            self.on_render_frame();

            crate::util::gl::funcs::check_gl_error("post-draw");

            self.window.swap_buffers();
        }
    }

    fn new(title: &str, width: u32, height: u32) -> Self {
        {
            info!(
                "Window created with params: {0} {1}x{2}",
                title, width, height
            );
            let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

            let (mut window, events) = glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Could not create window.");

            window.set_all_polling(true);
            window.make_current();
            gl::load_with(|s| glfw.get_proc_address_raw(s) as *const _);

            unsafe {
                gl::ClearColor(0., 0., 0., 0.0);
                gl::Enable(gl::DEPTH_TEST);
            }

            Self {
                glfw,
                window,
                event_polls: events,
                init_params: (title.to_string(), width, height),
            }
        }
    }
}
