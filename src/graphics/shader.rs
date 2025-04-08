use std::{collections::HashMap, marker::PhantomData};

use opengles::glesv2 as gl;

// useful:
//      https://github.com/ANtlord/glstudy/blob/master/src/main.rs
//      https://github.com/angular-rust/opengles-tutorial/blob/main/lesson-02/src/main.rs
//      https://github.com/Zephyros1938/ConsoleApp1/blob/main/Assets/Scripts/Shader.cs

static SHADERS_PATH: include_dir::Dir = include_dir::include_dir!("shaders");

pub struct Shader<DataType>
where
    DataType: Sized,
{
    program: gl::GLuint,
    phantom: PhantomData<DataType>,
    uniforms: UniformMap,
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
            program,
            phantom: PhantomData,
            uniforms: UniformMap::new(),
        }
    }

    pub fn useprogram(self) {
        gl::use_program(self.program);
    }

    pub fn get_attrib_location(self, name: &str) -> gl::GLint {
        gl::get_attrib_location(self.program, name)
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

struct UniformMap {
    uniforms: HashMap<String, Box<dyn std::any::Any>>,
}

impl UniformMap {
    pub fn new() -> Self {
        Self {
            uniforms: HashMap::new(),
        }
    }

    pub fn insert<T: Sized + 'static>(&mut self, name: String, data: T) {
        if !self.uniforms.contains_key(&name) {
            self.uniforms.insert(name, Box::new(data));
        } else {
            panic!("{}", format!("Could not insert uniform {}!", name));
        }
    }
}
