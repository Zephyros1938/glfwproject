use std::time::Duration;
// useful:
//      https://github.com/ANtlord/glstudy/blob/master/src/main.rs
//      https://github.com/angular-rust/opengles-tutorial/blob/main/lesson-02/src/main.rs

static SHADERS_PATH: include_dir::Dir = include_dir::include_dir!("shaders");

use glfw::*;
use opengles::glesv2 as gl;

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
        //gl::load_with(|s| window.get_proc_address(s) as *const _);
        window.make_current();
        window.set_cursor_pos_polling(true);

        gl::clear_color(1., 0.3, 0.3, 0.0);

        window.make_current();

        let sh: Shader<u8> = Shader::new("default.vert", "default.frag");

        Self {
            glfw,
            window,
            event_polls: events,
        }
    }
}

pub struct Shader<DataType>
where
    DataType: Sized,
{
    data: Vec<DataType>,
    program: gl::GLuint,
}

impl<DataType> Shader<DataType> {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_shader: gl::GLuint = shader_source(
            SHADERS_PATH
                .get_file(vertex_path)
                .expect(format!("Could not get file {}", vertex_path).as_str())
                .contents(),
            gl::GL_VERTEX_SHADER,
        );
        let fragment_shader: gl::GLuint = shader_source(
            SHADERS_PATH
                .get_file(fragment_path)
                .expect(format!("Could not get file {}", fragment_path).as_str())
                .contents(),
            gl::GL_FRAGMENT_SHADER,
        );
        let program = gl::create_program();
        gl::attach_shader(program, vertex_shader);
        gl::attach_shader(program, fragment_shader);
        gl::link_program(program);
        if gl::get_programiv(program, gl::GL_LINK_STATUS) == 0 {
            panic!("Could not link shader."); //TODO: add error catching
        };
        gl::detach_shader(program, vertex_shader);
        gl::detach_shader(program, fragment_shader);
        gl::delete_shader(vertex_shader);
        gl::delete_shader(fragment_shader);
        Self {
            data: vec![],
            program,
        }
    }
}

fn shader_source(source: &[u8], kind: gl::GLenum) -> gl::GLuint {
    let id = gl::create_shader(kind);
    gl::shader_source(id, &source);
    gl::compile_shader(id);

    if gl::get_shaderiv(id, gl::GL_COMPILE_STATUS) == 0 {
        panic!(
            "{}",
            format!(
                "Could not compile shader with reason {}\nSRC:\n{:?}",
                gl::get_shader_info_log(id, gl::get_shaderiv(id, gl::GL_INFO_LOG_LENGTH))
                    .expect("Could not get error"),
                source
            )
        );
    };
    id
}
