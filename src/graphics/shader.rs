use crate::graphics::utility;
use log::{debug, error, info, log, trace, warn};
use std::marker::PhantomData;

mod shader_cache;
mod uniform_map;

// useful:
//      https://github.com/ANtlord/glstudy/blob/master/src/main.rs
//      https://github.com/angular-rust/opengles-tutorial/blob/main/lesson-02/src/main.rs
//      https://github.com/Zephyros1938/ConsoleApp1/blob/main/Assets/Scripts/Shader.cs

static SHADERS_PATH: include_dir::Dir = include_dir::include_dir!("shaders");
pub type ShaderAnyData = dyn std::any::Any + 'static;

pub struct Shader<DataType>
where
    DataType: Sized + 'static,
{
    program: gl::types::GLuint,
    phantom: PhantomData<DataType>,
    uniforms: uniform_map::UniformMap,
    disposed: bool,
}

impl<DataType> Shader<DataType>
where
    DataType: Sized + 'static,
{
    pub unsafe fn new(vertex_path: &str, fragment_path: &str) -> Self {
        debug!(
            "Entering Shader::new with vertex_path: {} and fragment_path: {}",
            vertex_path, fragment_path
        );
        let vertex_shader: gl::types::GLuint = utility::shader_source(
            SHADERS_PATH
                .get_file(vertex_path)
                .expect(format!("Could not get file {}", vertex_path).as_str())
                .contents(),
            gl::VERTEX_SHADER,
        );
        debug!(
            "Vertex shader {} compiled successfully with id: {}",
            vertex_path, vertex_shader
        );
        let fragment_shader: gl::types::GLuint = utility::shader_source(
            SHADERS_PATH
                .get_file(fragment_path)
                .expect(format!("Could not get file {}", fragment_path).as_str())
                .contents(),
            gl::FRAGMENT_SHADER,
        );
        debug!(
            "Fragment shader {} compiled successfully with id: {}",
            fragment_path, fragment_shader
        );
        let program = gl::CreateProgram();
        debug!("Created program with id: {}", program);
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        debug!("Linked program with id: {}", program);
        if {
            let mut status = 1;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            status
        } == 0
        {
            error!("Shader linking failed for program {}", program);
            panic!("Could not link shader."); //TODO: add error catching
        };
        gl::DetachShader(program, vertex_shader);
        gl::DetachShader(program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
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

    pub unsafe fn useprogram(&self) {
        debug!("Entering Shader::useprogram with program: {}", self.program);
        gl::UseProgram(self.program);
        debug!(
            "Shader::useprogram executed using program: {}",
            self.program
        );
    }

    pub unsafe fn get_attrib_location(self, name: &str) -> gl::types::GLint {
        debug!(
            "Entering Shader::get_attrib_location for attribute: {}",
            name
        );
        let location = gl::GetAttribLocation(self.program, name.as_ptr() as *const i8);
        debug!(
            "Attribute {} location for program {} is: {}",
            name, self.program, location
        );
        location
    }

    pub unsafe fn dispose(mut self) {
        debug!("Entering Shader::dispose for program: {}", self.program);
        if !self.disposed {
            gl::DeleteProgram(self.program);
            self.disposed = true;
            info!("Shader disposed successfully for program: {}", self.program);
        } else {
            warn!(
                "Shader::dispose called but shader {} is already disposed",
                self.program
            );
        }
    }

    pub unsafe fn set_uniform<T: 'static>(&mut self, name: impl Into<String>, data: T) {
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
