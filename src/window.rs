use std::time::Duration;

use glfw::*;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

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
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    fn run(&mut self) {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);

            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);

            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERT.as_bytes().as_ptr().cast()),
                &(VERT.len().try_into().unwrap()),
            );

            gl::CompileShader(vertex_shader);

            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        }

        while !self.window.should_close() {
            self.event_logic();
            self.mainloop_logic();

            // The rest of the game loop goes here...

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
            gl::ClearColor(0.3, 0.3, 0.3, 0.0);
        }

        window.make_current();

        Self {
            glfw,
            window,
            event_polls: events,
        }
    }
}
