use opengles::glesv2 as gles;

pub fn gen_vertex_array() -> u32 {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    vao
}

pub fn shader_source(source: &[u8], kind: opengles::glesv2::GLenum) -> opengles::glesv2::GLuint {
    log::debug!("Entering shader_source for shader kind: {}", kind);
    let id = opengles::glesv2::create_shader(kind);
    log::debug!("Created shader id: {} for kind: {}", id, kind);
    opengles::glesv2::shader_source(id, &source);
    opengles::glesv2::compile_shader(id);
    log::debug!("Shader id: {} has been compiled", id);

    if opengles::glesv2::get_shaderiv(id, opengles::glesv2::GL_COMPILE_STATUS) == 0 {
        log::error!("Compilation failed for shader id: {}", id);
        panic!(
            "{}",
            format!(
                "Could not compile shader with reason {}\nSRC:\n{:?}",
                opengles::glesv2::get_shader_info_log(
                    id,
                    opengles::glesv2::get_shaderiv(id, opengles::glesv2::GL_INFO_LOG_LENGTH)
                )
                .expect("Could not get error"),
                source
            )
        );
    }
    log::debug!("Exiting shader_source successfully with shader id: {}", id);
    id
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
            GL_DrawMode::POINTS => gles::GL_POINTS,
            GL_DrawMode::TRIANGLES => gles::GL_TRIANGLES,
            GL_DrawMode::TRIANGLE_STRIP => gles::GL_TRIANGLE_STRIP,
            GL_DrawMode::TRIANGLE_FAN => gles::GL_TRIANGLE_FAN,
            GL_DrawMode::LINES => gles::GL_LINES,
            GL_DrawMode::LINE_LOOP => gles::GL_LINE_LOOP,
            GL_DrawMode::LINE_STRIP => gles::GL_LINE_STRIP,
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
        gles::GL_FLOAT
    }
}
