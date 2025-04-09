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
