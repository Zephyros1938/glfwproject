use crate::graphics::{shader::*, utility};
use gl;
use log::{debug, error, info, log, trace, warn};
use std::collections::HashMap;

mod shader_program_cache;

pub struct shader_program<DataType: utility::GL_DataType + 'static> {
    shader: Shader<DataType>,
    vao: u32,
    drawmode: utility::GL_DrawMode,
    buffers: HashMap<String, u32>,
}

impl<DataType: utility::GL_DataType + 'static> shader_program<DataType> {
    pub fn new(vertex_path: &str, fragment_path: &str, drawmode: utility::GL_DrawMode) -> Self {
        Self {
            shader: Shader::new(vertex_path, fragment_path),
            vao: utility::gen_vertex_array(),
            drawmode,
            buffers: HashMap::new(),
        }
    }

    pub fn set_array(&mut self, name: String, position: u32, datasize: i32, data: &mut [DataType]) {
        let id = {
            let mut buf = 0;
            unsafe {
                gl::GenBuffers(1, &mut buf);
            }
            buf
        };
        self.buffers.insert(name, id);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * size_of::<DataType>()) as isize,
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
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) };
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(self.drawmode.value(), 0, 3);
        };
    }

    pub fn useshader(&self) {
        self.shader.useprogram();
    }
}
