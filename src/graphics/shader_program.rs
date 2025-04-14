#[allow(unused_imports)]
use crate::graphics::shader::*;
use crate::util::gl::enums::{DataType, DrawMode};
use crate::util::gl::funcs::gen_vertex_array;
use gl;
use log::debug;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ShaderProgram {
    shader: Shader,
    vao: u32,
    drawmode: DrawMode,
    buffers: HashMap<String, (u32, i32)>,
    vertex_len: i32,
}

impl ShaderProgram {
    pub fn new(vertex_path: &str, fragment_path: &str, drawmode: DrawMode) -> Self {
        debug!(
            "Entering ShaderProgram::new with vertex_path: {} and fragment_path: {}",
            vertex_path, fragment_path
        );
        Self {
            shader: Shader::new(vertex_path, fragment_path),
            vao: gen_vertex_array(),
            drawmode,
            buffers: HashMap::new(),
            vertex_len: 0,
        }
    }

    pub fn set_array<DT: DataType + 'static>(
        &mut self,
        name: String,
        position: u32,
        datasize: i32,
        data: &mut [DT],
    ) {
        self.bind();
        let id = {
            let mut buf = 0;
            unsafe {
                gl::GenBuffers(1, &mut buf);
            }
            buf
        };
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * size_of::<DT>()) as isize,
                data.as_mut_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                position,
                datasize,
                data[0].value(),
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(position);
        }
        self.buffers.insert(name, (id, datasize));
        self.unbind();
    }
    pub fn set_vertex<DT: DataType + 'static>(
        &mut self,
        name: String,
        position: u32,
        datasize: i32,
        data: &mut [DT],
    ) {
        self.set_array(name, position, datasize, data);
        self.vertex_len = data.len() as i32;
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) };
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) };
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(self.drawmode.value(), 0, self.vertex_len);
        };
    }

    pub fn useshader(&self) {
        self.shader.useprogram();
    }

    pub fn dispose(&mut self) {
        let vao_id = self.vao;
        debug!(
            "Entering ShaderProgram::dispose for ShaderProgram {}",
            self.vao
        );
        unsafe {
            for (name, (id, _)) in &self.buffers {
                debug!("Deleting buffer {} ({})", name, id);
                gl::DeleteBuffers(1, id);
            }
            debug!("Deleting VAO {}", &self.vao);
            gl::DeleteVertexArrays(1, &self.vao);
        }
        self.shader.dispose();
        self.unbind();
        debug!(
            "ShaderProgram::dispose completed successfully for ShaderProgram {}",
            vao_id
        );
    }
    pub fn uniform_matrix4x4(&self, name: String, data: &crate::math::matrix4x4::Matrix4x4) {
        self.bind();
        self.useshader();
        self.shader
            .set_uniform_matrix4x4(name.as_str(), data)
            .unwrap();
        self.unbind();
    }
    pub fn set_drawmode(&mut self, drawmode: DrawMode) {
        self.drawmode = drawmode;
    }
}
