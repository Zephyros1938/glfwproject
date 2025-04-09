use crate::graphics::shader::*;
use std::collections::HashMap;

pub struct ShaderCache {
    shaders: HashMap<u32, Shader<Box<ShaderAnyData>>>,
}
