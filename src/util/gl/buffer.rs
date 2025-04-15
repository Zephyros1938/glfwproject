use log::{trace, warn};

pub struct Buffer {
    pub id: u32,
    pub target: u32,
}

impl Buffer {
    pub fn new(target: u32) -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        Self { id, target }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    pub fn buffer_data<T>(&self, data: &[T], usage: u32) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            warn!("Dropping buffer {} : {}", self.id, self.target);
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
