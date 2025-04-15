use std::ptr;

use crate::graphics::shader_program::ShaderProgramIndiced;
use crate::graphics::shader_program::ShaderProgramMap;
use crate::math::math3d::camera::Camera;
use crate::math::math3d::eye::EyeBase;
use crate::math::*;
use crate::util::game::event::Event;
use crate::util::mesh::raw::Mesh;
use crate::window::WindowBase;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, fail_on_errors};
use log::{debug, info};

use super::frame_event_args::FrameEventArgs;

pub struct GameWindow {
    camera: Camera,
    shader_list: ShaderProgramMap<ShaderProgramIndiced>,
    _glfw: Glfw,
    window: PWindow,
    event_polls: GlfwReceiver<(f64, WindowEvent)>,
    delta_time: super::delta_time::DeltaTime,
    event_list: Vec<Event<!>>,
    mouse_locked: bool,
}

impl GameWindow {
    fn on_update_frame(&mut self, _e: FrameEventArgs) {
        if let Err(e) =
            self.shader_list["test.main"].set_uniform_matrix4x4("view", &self.camera.get_view())
        {
            eprintln!("Error setting uniform: {}", e);
        }
        crate::util::gl::funcs::check_gl_error("view-uniform");
    }

    fn on_render_frame(&mut self, _e: FrameEventArgs) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
        for (_, _shader) in self.shader_list.get_all_1() {
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

                    if let Err(e) = self.shader_list["test.main"]
                        .set_uniform_matrix4x4("projection", &self.camera.eye.projection)
                    {
                        eprintln!("Error setting uniform: {}", e);
                    }
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
                    // Key::Num1 => self.shader_list["test.main"].set_drawmode(DrawMode::TRIANGLES),
                    // Key::Num2 => self.shader_list["test.main"].set_drawmode(DrawMode::LINES),
                    // Key::Num3 => self.shader_list["test.main"].set_drawmode(DrawMode::POINTS),
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
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::TEXTURE_2D);
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
        let mut sh = ShaderProgramIndiced::builder("shaders/default.vert", "shaders/default.frag")
            .add_attribute(
                0,                                  // Attribute location for position
                3,                                  // Number of components (x, y, z)
                gl::FLOAT,                          // Type: float
                gl::FALSE,                          // Not normalized
                (6 * size_of::<f32>()) as i32, // Stride: total size of one vertex (position + color)
                (0 * size_of::<f32>()) as *const _, // Offset: position starts at the beginning
            )
            .add_attribute(
                1,                                  // Attribute location for color
                3,                                  // Number of components (r, g, b)
                gl::FLOAT,                          // Type: float
                gl::FALSE,                          // Not normalized
                (6 * size_of::<f32>()) as i32,      // Stride: total size of one vertex
                (3 * size_of::<f32>()) as *const _, // Offset: color data starts after the first three floats (position)
            )
            .build();
        let mesh = Mesh::new(&[
            // Each vertex: [pos.x, pos.y, pos.z,  r, g, b]
            -0.5f32, -0.5f32, 0.0f32, 1.0, 0.0, 0.0, // Vertex 1: Red
            0.5f32, -0.5f32, 0.0f32, 0.0, 1.0, 0.0, // Vertex 2: Green
            0.0f32, 0.5f32, 0.0f32, 0.0, 0.0, 1.0, // Vertex 3: Blue
        ]);

        sh.set_indices_mesh(mesh);

        let model_matrix =
            matrix4x4::Matrix4x4::create_rotation_x(mathhelper::degrees_to_radians_f32(0f32));
        if let Err(e) = sh.set_uniform_matrix4x4("model", &model_matrix) {
            eprintln!("Error setting uniform: {}", e);
        }
        if let Err(e) = sh.set_uniform_matrix4x4("view", &self.camera.get_view()) {
            eprintln!("Error setting uniform: {}", e);
        }
        if let Err(e) = sh.set_uniform_matrix4x4("projection", &self.camera.eye.projection) {
            eprintln!("Error setting uniform: {}", e);
        }

        // Define the positions for each triangle of the cube's 6 faces.
        // Each face is composed of 2 triangles (6 vertices).
        // The cube is centered at the origin with sides of length 1.
        // sh.set_vertex(
        //     "aPosition".to_string(),
        //     0,
        //     3,
        //     &mut crate::util::gl::example_shader_values::cube::VERTICES_CW.clone(),
        // );

        // Define colors for each vertex. Here each face is given a unique color.
        // Notice that each color is repeated 6 times for the six vertices
        // corresponding to that face.
        // sh.set_array(
        //     "aColor".to_string(),
        //     1,
        //     3,
        //     &mut crate::util::gl::example_shader_values::cube::COLORS.clone(),
        // );

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
            // shader.clone().dispose();
        }
    }
}
