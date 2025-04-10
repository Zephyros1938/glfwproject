pub fn gen_vertex_array() -> u32 {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    vao
}

pub unsafe fn shader_source(source: &[u8], kind: gl::types::GLenum) -> gl::types::GLuint {
    log::debug!("Entering shader_source for shader kind: {}", kind);
    let id = gl::CreateShader(kind);
    log::debug!("Created shader id: {} for kind: {}", id, kind);
    let c_str = std::ffi::CString::new(source).expect("Shader source had interior null bytes");
    let ptr = c_str.as_ptr();

    gl::ShaderSource(id, 1, &ptr, std::ptr::null());
    gl::CompileShader(id);
    log::debug!("Shader id: {} has been compiled", id);

    if get_shaderiv(id, gl::COMPILE_STATUS) == 0 {
        log::error!("Compilation failed for shader id: {}", id);
        panic!(
            "{}",
            format!(
                "Could not compile shader with reason {}",
                get_shader_info_log(id).expect("Could not get error")
            )
        );
    }
    log::debug!("Exiting shader_source successfully with shader id: {}", id);
    id
}

pub unsafe fn get_shaderiv(id: gl::types::GLuint, pname: gl::types::GLenum) -> gl::types::GLint {
    let mut params = 1;
    gl::GetShaderiv(id, pname, &mut params);
    params
}

pub unsafe fn get_shader_info_log(shader: gl::types::GLuint) -> Result<String, ()> {
    // First, query the length of the info log.
    let mut log_length: gl::types::GLint = 0;
    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

    // Check if there's any info log.
    if log_length <= 0 {
        return Err(());
    }

    // Allocate buffer with enough capacity to hold the log.
    // We add 1 for the null terminator.
    let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize + 1);
    // Fill the buffer with zeros. This ensures it’s null terminated.
    buffer.extend([0].iter().cycle().take(log_length as usize + 1));

    // Retrieve the log.
    gl::GetShaderInfoLog(
        shader,
        log_length,
        std::ptr::null_mut(), // We're ignoring the actual length returned; it's log_length.
        buffer.as_mut_ptr() as *mut gl::types::GLchar,
    );

    // Convert the C-style string (null terminated) to a Rust String.
    // We first find the first null terminator, if any.
    if let Some(null_pos) = buffer.iter().position(|&c| c == 0) {
        buffer.truncate(null_pos);
    }

    // Convert UTF-8 encoded data to a String.
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

pub enum GL_DrawMode {
    POINTS = 0,
    TRIANGLES = 100,
    TRIANGLE_STRIP = 101,
    TRIANGLE_FAN = 102,
    LINES = 300,
    LINE_LOOP = 301,
    LINE_STRIP = 302,
}

impl GL_DrawMode {
    pub fn value(&self) -> u32 {
        match self {
            GL_DrawMode::POINTS => gl::POINTS,
            GL_DrawMode::TRIANGLES => gl::TRIANGLES,
            GL_DrawMode::TRIANGLE_STRIP => gl::TRIANGLE_STRIP,
            GL_DrawMode::TRIANGLE_FAN => gl::TRIANGLE_FAN,
            GL_DrawMode::LINES => gl::LINES,
            GL_DrawMode::LINE_LOOP => gl::LINE_LOOP,
            GL_DrawMode::LINE_STRIP => gl::LINE_STRIP,
        }
    }
    pub fn default() -> Self {
        GL_DrawMode::TRIANGLES
    }
}

pub trait GL_DataType {
    fn value(&self) -> u32;
}

impl GL_DataType for f32 {
    fn value(&self) -> u32 {
        gl::FLOAT
    }
}

pub unsafe fn check_gl_error(location: &str) {
    let error = gl::GetError();
    if error != gl::NO_ERROR {
        println!("OpenGL error at {}: {}", location, error);
    }
}
