use log::{debug, error, info, log, trace, warn};
use opengles::glesv2 as gl;
use std::{
    any::{Any, type_name},
    marker::PhantomData,
};

mod shader_cache;
mod uniform_map;
mod utility;

// useful:
//      https://github.com/ANtlord/glstudy/blob/master/src/main.rs
//      https://github.com/angular-rust/opengles-tutorial/blob/main/lesson-02/src/main.rs
//      https://github.com/Zephyros1938/ConsoleApp1/blob/main/Assets/Scripts/Shader.cs

static SHADERS_PATH: include_dir::Dir = include_dir::include_dir!("shaders");
type ShaderAnyData = dyn std::any::Any;

pub struct Shader<DataType>
where
    DataType: Sized + 'static,
{
    program: gl::GLuint,
    phantom: PhantomData<DataType>,
    uniforms: uniform_map::UniformMap,
    disposed: bool,
}

impl<DataType> Shader<DataType>
where
    DataType: Sized + 'static,
{
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        debug!(
            "Entering Shader::new with vertex_path: {} and fragment_path: {}",
            vertex_path, fragment_path
        );
        let vertex_shader: gl::GLuint = utility::shader_source(
            SHADERS_PATH
                .get_file(vertex_path)
                .expect(format!("Could not get file {}", vertex_path).as_str())
                .contents(),
            gl::GL_VERTEX_SHADER,
        );
        debug!(
            "Vertex shader {} compiled successfully with id: {}",
            vertex_path, vertex_shader
        );
        let fragment_shader: gl::GLuint = utility::shader_source(
            SHADERS_PATH
                .get_file(fragment_path)
                .expect(format!("Could not get file {}", fragment_path).as_str())
                .contents(),
            gl::GL_FRAGMENT_SHADER,
        );
        debug!(
            "Fragment shader {} compiled successfully with id: {}",
            fragment_path, fragment_shader
        );
        let program = gl::create_program();
        debug!("Created program with id: {}", program);
        gl::attach_shader(program, vertex_shader);
        gl::attach_shader(program, fragment_shader);
        gl::link_program(program);
        debug!("Linked program with id: {}", program);
        if gl::get_programiv(program, gl::GL_LINK_STATUS) == 0 {
            error!("Shader linking failed for program {}", program);
            panic!("Could not link shader."); //TODO: add error catching
        };
        gl::detach_shader(program, vertex_shader);
        gl::detach_shader(program, fragment_shader);
        gl::delete_shader(vertex_shader);
        gl::delete_shader(fragment_shader);
        debug!("Detached and deleted shaders for program {}", program);
        let shader_instance = Self {
            program,
            phantom: PhantomData,
            uniforms: uniform_map::UniformMap::new(),
            disposed: false,
        };
        debug!(
            "Shader::new completed successfully with program: {}",
            shader_instance.program
        );
        shader_instance
    }

    pub fn useprogram(self) {
        debug!("Entering Shader::useprogram with program: {}", self.program);
        gl::use_program(self.program);
        debug!(
            "Shader::useprogram executed using program: {}",
            self.program
        );
    }

    pub fn get_attrib_location(self, name: &str) -> gl::GLint {
        debug!(
            "Entering Shader::get_attrib_location for attribute: {}",
            name
        );
        let location = gl::get_attrib_location(self.program, name);
        debug!(
            "Attribute {} location for program {} is: {}",
            name, self.program, location
        );
        location
    }

    pub fn dispose(mut self) {
        debug!("Entering Shader::dispose for program: {}", self.program);
        if !self.disposed {
            gl::delete_program(self.program);
            self.disposed = true;
            info!("Shader disposed successfully for program: {}", self.program);
        } else {
            warn!(
                "Shader::dispose called but shader {} is already disposed",
                self.program
            );
        }
    }

    pub fn set_uniform<T: 'static>(&mut self, name: impl Into<String>, data: T) {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<DataType>() {
            let key: String = name.into();
            match self.uniforms.insert(key, data) {
                Ok(_) => (),
                Err(reason) => {
                    error!("{}", reason);
                }
            }
        }
    }
}
