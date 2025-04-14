use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::size_of;

use crate::{
    graphics::shader::Shader,
    util::gl::{buffer::Buffer, enums::DrawMode, vertex_array::VertexArray},
};

pub struct ShaderProgramIndiced {
    shader: Shader,
    vao: u32,
    ebo: u32,
    drawmode: gl::types::GLenum,
    buffers: HashMap<String, (u32, i32)>,
    indice_len: u32,
}

impl ShaderProgramIndiced {
    /// Returns a builder to configure a new ShaderProgramIndiced.
    pub fn builder(vertex_path: &str, fragment_path: &str) -> ShaderProgramBuilder {
        ShaderProgramBuilder::new(vertex_path, fragment_path)
    }

    /// Uploads the mesh's vertex and index data to the GPU.
    /// Assumes that `mesh.get_data()` returns a slice of vertices and that
    /// `mesh.get_indices()` returns a slice of u32 indices.
    pub fn set_indices<T: Clone + std::fmt::Debug + PartialEq>(
        &mut self,
        mesh: crate::util::mesh::raw::Mesh<T>,
    ) {
        // Bind the VAO so that the new buffers get associated with it.
        unsafe {
            gl::BindVertexArray(self.vao);
        }

        // Create and upload vertex data.
        let vbo = {
            let mut buf = 0;
            unsafe {
                gl::GenBuffers(1, &mut buf);
            }
            buf
        };
        unsafe {
            let vertices = mesh.get_data();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<T>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        // Upload index data.
        unsafe {
            let indices = mesh.get_indices();
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
            // Update the indice_len field so the draw call knows how many indices to render.
            self.indice_len = indices.len() as u32;
        }

        // Unbind the VAO (optional).
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Activates the shader program.
    pub fn useprogram(&self) {
        self.shader.useprogram();
    }

    /// Draws the geometry using the stored VAO, EBO and shader.
    pub fn draw(&self) {
        // Bind the VAO, activate the shader and draw.
        unsafe {
            gl::BindVertexArray(self.vao);
        }
        self.useprogram();
        unsafe {
            gl::DrawElements(
                self.drawmode,
                self.indice_len as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

    /// Sets a 4x4 matrix uniform in the shader.
    pub fn set_uniform_matrix4x4(
        &self,
        name: &str,
        data: &crate::math::matrix4x4::Matrix4x4,
    ) -> Result<(), super::errors::UniformError> {
        // Assuming Shader::set_uniform_matrix4x4 returns a Result<(), String>.
        let result = self.shader.set_uniform_matrix4x4(name, data);
        let err = unsafe { gl::GetError() };
        if err != gl::NO_ERROR {
            eprintln!("OpenGL error after setting uniform: {:#X}", err);
        }
        result
    }

    // /// Sets an integer uniform in the shader.
    // pub fn set_uniform_int(&self, name: &str, value: i32) -> Result<(), String> {
    //     unsafe {
    //         gl::BindVertexArray(self.vao);
    //     }
    //     self.useprogram();
    //     // Assuming Shader::set_uniform_int returns a Result<(), String>.
    //     let result = self.shader.set_uniform_int(name, value);
    //     unsafe {
    //         gl::BindVertexArray(0);
    //     }
    //     result
    // }
}

impl super::ShaderProgram for ShaderProgramIndiced {}

pub struct ShaderProgramBuilder {
    shader: Shader,
    drawmode: u32,
    // temporary storage for additional configuration (attributes)
    attribute_configs: Vec<AttributeConfig>,
}

struct AttributeConfig {
    index: u32,
    size: i32,
    data_type: u32,
    normalized: u8,
    stride: i32,
    offset: *const c_void,
}

impl ShaderProgramBuilder {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        Self {
            shader: Shader::new(vertex_path, fragment_path),
            drawmode: gl::TRIANGLES,
            attribute_configs: Vec::new(),
        }
    }

    pub fn drawmode(mut self, mode: u32) -> Self {
        self.drawmode = mode;
        self
    }

    pub fn add_attribute(
        mut self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: u8,
        stride: i32,
        offset: *const c_void,
    ) -> Self {
        self.attribute_configs.push(AttributeConfig {
            index,
            size,
            data_type,
            normalized,
            stride,
            offset,
        });
        self
    }

    pub fn build(self) -> ShaderProgramIndiced {
        let vao = VertexArray::new(); // A RAII type for VAO.
        vao.bind();
        for attr in self.attribute_configs {
            unsafe {
                gl::VertexAttribPointer(
                    attr.index,
                    attr.size,
                    attr.data_type,
                    attr.normalized,
                    attr.stride,
                    attr.offset,
                );
                gl::EnableVertexAttribArray(attr.index);
            }
        }
        vao.unbind();
        ShaderProgramIndiced {
            shader: self.shader,
            vao: vao.id,
            ebo: {
                let buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                buffer.id // In the future, you might store the whole Buffer object.
            },
            drawmode: self.drawmode,
            buffers: HashMap::new(),
            indice_len: 0,
        }
    }
}
