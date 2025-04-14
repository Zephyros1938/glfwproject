pub fn gen_vertex_array() -> u32 {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    vao
}

pub fn create_shader(kind: gl::types::GLenum) -> gl::types::GLuint {
    log::debug!("Entering shader with kind: {}", kind);
    let shader = unsafe { gl::CreateShader(kind) };
    log::debug!("Created shader id: {} for kind: {}", shader, kind);
    shader
}

pub fn shader_source(source: std::ffi::CString, shader: gl::types::GLuint) {
    log::debug!("Entering shader_source for shader: {}", shader);

    unsafe { gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null()) };
    unsafe { gl::CompileShader(shader) };

    if get_shaderiv(shader, gl::COMPILE_STATUS) == 0 {
        log::error!("Compilation failed for shader id: {}", shader);
        panic!(
            "{}",
            format!(
                "Could not compile shader with reason {}",
                get_shader_info_log(shader).expect("Could not get error")
            )
        );
    } else {
        log::debug!("Shader id: {} has been compiled", shader);
    }
    log::debug!(
        "Exiting shader_source successfully with shader id: {}",
        shader
    );
}

pub fn get_shaderiv(id: gl::types::GLuint, pname: gl::types::GLenum) -> gl::types::GLint {
    let mut params = 1;
    unsafe { gl::GetShaderiv(id, pname, &mut params) };
    params
}

pub fn get_shader_info_log(shader: gl::types::GLuint) -> Result<String, ()> {
    let mut log_length: gl::types::GLint = 0;
    unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length) };

    if log_length <= 0 {
        return Err(());
    }

    // allocate a buffer with enough capacity to hold the log.
    let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize + 1);
    buffer.extend([b' '].iter().cycle().take(log_length as usize));

    unsafe {
        gl::GetShaderInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(), // We're ignoring the actual length returned; it's log_length.
            buffer.as_ptr() as *mut gl::types::GLchar,
        )
    };

    if let Some(null_pos) = buffer.iter().position(|&c| c == 0) {
        buffer.truncate(null_pos);
    }

    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

pub fn get_uniform_location(program: gl::types::GLuint, name: &str) -> gl::types::GLint {
    let c_name = std::ffi::CString::new(name).unwrap();
    let location = unsafe { gl::GetUniformLocation(program, c_name.as_ptr()) };
    if location == -1 {
        log::error!("Uniform {} not found in program {}", name, program);
    }
    location
}

pub fn check_gl_error(location: &str) {
    let error = unsafe { gl::GetError() };
    if error != gl::NO_ERROR {
        log::error!("OpenGL error at {}: {}", location, error);
    }
}
