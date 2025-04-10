use crate::graphics::shader::*;
use std::collections::HashMap;

pub type ShaderAnyData = dyn std::any::Any + 'static;

pub struct ShaderCache {
    shaders: HashMap<u32, Shader<Box<ShaderAnyData>>>,
}
