use glfw::{
    Action, Context, Cursor, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, fail_on_errors,
};
use log::{debug, info};

use crate::util::game::event::Event;

use crate::graphics::{shader_program::ShaderProgram, shader_program_map::ShaderProgramMap};
use crate::math::math3d::camera::Camera;
use crate::math::math3d::eye::EyeBase;
use crate::math::*;
use crate::window::WindowBase;

use super::frame_event_args::FrameEventArgs;

pub struct GameWindow {
    camera: Camera,
    shader_list: ShaderProgramMap,
    _glfw: Glfw,
    window: PWindow,
    event_polls: GlfwReceiver<(f64, WindowEvent)>,
    delta_time: super::delta_time::DeltaTime,
    event_list: Vec<Event<!>>,
    mouse_locked: bool,
}

impl GameWindow {
    fn on_update_frame(&mut self, _e: FrameEventArgs) {
        self.shader_list["test.main"]
            .uniform_matrix4x4("view".to_string(), &self.camera.get_view());
        crate::util::gl::funcs::check_gl_error("view-uniform");
    }

    fn on_render_frame(&mut self, _e: FrameEventArgs) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
        for (_, _shader) in self.shader_list.get_all_1() {
            _shader.bind();
            _shader.useshader();
            _shader.draw();
        }
    }

    /// Polled event handling, such as a singular keypress, framebuffer resize, or click.
    /// 
    /// 
    fn event_logic(&mut self, _e: FrameEventArgs) {
        self.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_polls) {
            if let glfw::WindowEvent::FramebufferSize(x, y) = event {
                unsafe {
                    gl::Viewport(0, 0, x, y);
                    self.camera.set_aspect_ratio(x as f32 / y as f32);
                    self.shader_list["test.main"]
                        .uniform_matrix4x4("projection".to_string(), &self.camera.eye.projection);
                }
            }
            match event {
                glfw::WindowEvent::Key(k, _, Action::Press, _) => match k {
                    Key::E => {
                        self.mouse_locked = !self.mouse_locked;
                        if self.mouse_locked {
                            self.window.set_cursor_mode(glfw::CursorMode::Disabled);
                        } else {
                            self.window.set_cursor_mode(glfw::CursorMode::Normal);
                        }
                    }
                    Key::F11 => {
                        self.window.maximize();
                    }
                    _ => {}
                },
                _ => (),
            }
        }
        self.handle_input(_e);
    }

    /// Constant input handling, such as player movement.
    fn handle_input(&mut self, frame_event: FrameEventArgs) {
        // Instead of handling only on key events, continuously poll the key states:
        if self.window.get_key(Key::W) == Action::Press {
            self.camera.forward(frame_event.time);
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.camera.backward(frame_event.time);
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.camera.left(frame_event.time);
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.camera.right(frame_event.time);
        }
        if self.window.get_key(Key::Space) == Action::Press {
            self.camera.up(frame_event.time);
        }
        if self.window.get_key(Key::LeftControl) == Action::Press {
            self.camera.down(frame_event.time);
        }
        // Handle other keys such as Escape to close the window
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true);
        }
        if self.window.is_focused() && self.mouse_locked {
            let (x, y) = self.window.get_cursor_pos();
            self.camera.update([x as f32, y as f32]);
            self.camera.rotate();
        }
    }
}

impl WindowBase for GameWindow {
    fn new(title: &str, width: u32, height: u32) -> Self {
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

        let delta_time = super::delta_time::DeltaTime::new(glfw.get_time());

        GameWindow {
            camera: crate::math::math3d::camera::Camera::new(
                vector3::Vector3::from(0f32, 0f32, -3f32),
                matrix4x4::Matrix4x4::create_perspective_fov(
                    45f32,
                    100f32 / 100f32,
                    0.01f32,
                    100f32,
                ),
                5f32,
                0.1f32,
                800f32 / 600f32,
            ),
            shader_list: ShaderProgramMap::new(),
            _glfw: glfw,
            window,
            event_polls: events,
            delta_time,
            event_list: Vec::new(),
            mouse_locked: false,
        }
    }

    fn run(&mut self) {
        self.pre_load();
        while !self.window.should_close() {
            self.delta_time.update(self._glfw.get_time());
            let frame_event_args =
                FrameEventArgs::new(self.delta_time.get(), self.delta_time.total());
            self.event_logic(frame_event_args);
            self.on_update_frame(frame_event_args);
            self.on_render_frame(frame_event_args);
            self.window.swap_buffers();
        }
    }

    fn pre_load(&mut self) {
        let mut sh: ShaderProgram = ShaderProgram::new(
            "shaders/default.vert",
            "shaders/default.frag",
            crate::util::gl::enums::DrawMode::TRIANGLES,
        );
        let model_matrix =
            matrix4x4::Matrix4x4::create_rotation_x(mathhelper::degrees_to_radians_f32(0f32));
        sh.uniform_matrix4x4("model".to_string(), &model_matrix);
        crate::util::gl::funcs::check_gl_error("model-uniform");
        sh.uniform_matrix4x4("view".to_string(), &self.camera.get_view());
        crate::util::gl::funcs::check_gl_error("view-uniform");
        sh.uniform_matrix4x4("projection".to_string(), &self.camera.eye.projection);
        crate::util::gl::funcs::check_gl_error("projection-uniform");

        sh.set_vertex(
            "aPosition".to_string(),
            0,
            3,
            &mut [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
        );

        sh.set_array(
            "aColor".to_string(),
            1,
            3,
            &mut [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        );
        self.shader_list.add("test.main".to_string(), true, sh);

        let windowsize = self.window.get_size();
        self.window.set_monitor(
            glfw::WindowMode::Windowed,
            0,
            0,
            windowsize.0 as u32,
            windowsize.1 as u32,
            Some(60),
        );
    }

    fn unload(&mut self) {
        for (name, enabled, shader) in self.shader_list.get_all() {
            debug!("Unloading shader {} with enabled status {}", name, enabled);
            shader.clone().dispose();
        }
    }
}
