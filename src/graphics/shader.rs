use crate::math::matrix4x4::Matrix4x4;
use crate::util::gl::funcs::*;
use log::{debug, error, info};
use std::ffi::CString;

pub mod errors;
mod shader_cache;
mod uniform_map;

// useful:
//      https://github.com/ANtlord/glstudy/blob/master/src/main.rs
//      https://github.com/angular-rust/opengles-tutorial/blob/main/lesson-02/src/main.rs
//      https://github.com/Zephyros1938/ConsoleApp1/blob/main/Assets/Scripts/Shader.cs
//      https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html
//      https://github.com/Z-1938Studios/VoxelGame1/blob/main/Assets/Scripts/Game.cs

#[derive(Clone)]
pub struct Shader {
    program: gl::types::GLuint,
    disposed: bool,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        debug!(
            "Entering Shader::new with vertex_path: {} and fragment_path: {}",
            vertex_path, fragment_path
        );
        let vertex_shader: gl::types::GLuint = create_shader(gl::VERTEX_SHADER);

        shader_source(
            crate::util::asset_management::read_asset_to_cstr(vertex_path),
            vertex_shader,
        );

        debug!(
            "Vertex shader {} compiled successfully with id: {}",
            vertex_path, vertex_shader
        );
        let fragment_shader: gl::types::GLuint = create_shader(gl::FRAGMENT_SHADER);

        shader_source(
            crate::util::asset_management::read_asset_to_cstr(fragment_path),
            fragment_shader,
        );

        debug!(
            "Fragment shader {} compiled successfully with id: {}",
            fragment_path, fragment_shader
        );
        let program = unsafe { gl::CreateProgram() };
        debug!("Created program with id: {}", program);
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        };
        debug!("Linked program with id: {}", program);
        if {
            let mut status = 1;
            unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut status) };
            status
        } == 0
        {
            error!("Shader linking failed for program {}", program);
            panic!("Could not link shader."); //TODO: add error catching
        };
        unsafe {
            gl::DetachShader(program, vertex_shader);
            gl::DetachShader(program, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        };
        debug!("Detached and deleted shaders for program {}", program);
        let shader_instance = Self {
            program,
            // uniforms: uniform_map::UniformMap::new(),
            disposed: false,
        };
        debug!(
            "Shader::new completed successfully with program: {}",
            shader_instance.program
        );
        shader_instance
    }

    pub fn useprogram(&self) {
        unsafe { gl::UseProgram(self.program) }
    }

    pub fn get_attrib_location(self, name: &str) -> gl::types::GLint {
        debug!(
            "Entering Shader::get_attrib_location for attribute: {}",
            name
        );
        let cname = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetAttribLocation(self.program, cname.as_ptr()) };
        debug!(
            "Attribute {} location for program {} is: {}",
            name, self.program, location
        );
        location
    }

    pub fn dispose(&mut self) {
        debug!("Entering Shader::dispose for program: {}", self.program);
        if !self.disposed {
            unsafe { gl::DeleteProgram(self.program) };
            self.disposed = true;
            info!("Shader disposed successfully for program: {}", self.program);
        } else {
            error!(
                "Shader::dispose called but shader {} is already disposed",
                self.program
            );
        }
    }

    pub fn set_uniform_matrix4x4(
        &self,
        name: &str,
        data: &Matrix4x4,
    ) -> Result<(), errors::UniformError> {
        // Ensure the shader program is active.

        let c_name = CString::new(name)?;
        let location = unsafe { gl::GetUniformLocation(self.program, c_name.as_ptr()) };

        if location == -1 {
            return Err(errors::UniformError::NotFound {
                name: name.to_string(),
                program_id: self.program,
            });
        }

        // Set the matrix uniform. Use gl::FALSE if your matrix is column-major.
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, data.as_ptr());
        }

        Ok(())
    }
}
