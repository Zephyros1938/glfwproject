use log::{debug, trace, warn};

pub struct VertexArray {
    pub id: u32,
}

impl VertexArray {
    /// Creates a new VertexArray object by generating an OpenGL VAO.
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    /// Binds the VAO.
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbinds any VAO.
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            warn!("Dropping VAO {}", self.id);
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
