use std::collections::HashMap;

use log::debug;

use super::{ShaderProgram, ShaderProgramVertex};

pub struct ShaderProgramMap<T>
where
    T: ShaderProgram,
{
    shaders: HashMap<String, (bool, T)>,
}

impl<T: ShaderProgram> ShaderProgramMap<T> {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: String, enabled: bool, shader: T) {
        self.shaders.insert(name, (enabled, shader));
    }
    pub fn rem(&mut self, name: &str) {
        self.shaders.remove(name);
    }
    pub fn enable_shader(&mut self, name: &str) {
        if let Some((enabled, _)) = self.shaders.get_mut(name) {
            *enabled = true;
            debug!("Enabled shader: {}", name);
        }
    }
    pub fn disable_shader(&mut self, name: &str) {
        if let Some((enabled, _)) = self.shaders.get_mut(name) {
            *enabled = false;
            debug!("Disabled shader: {}", name);
        }
    }
    pub fn toggle_shader(&mut self, name: &str) {
        if let Some((enabled, _)) = self.shaders.get_mut(name) {
            *enabled = !*enabled;
            debug!("Toggled shader [{}] to {}", name, *enabled);
        }
    }

    pub fn get_shader(&self, name: &str) -> Option<&T> {
        self.shaders.get(name).map(|(_, shader)| shader)
    }
    pub fn get_shader_mut(&mut self, name: &str) -> Option<&mut T> {
        self.shaders.get_mut(name).map(|(_, shader)| shader)
    }
    pub fn get_all_1(&self) -> Vec<(String, &T)> {
        self.shaders
            .iter()
            .filter(|(_, (enabled, _))| *enabled)
            .map(|(name, (_, shader))| (name.clone(), shader))
            .collect()
    }
    pub fn get_all_0(&self) -> Vec<(String, &T)> {
        self.shaders
            .iter()
            .filter(|(_, (enabled, _))| !*enabled)
            .map(|(name, (_, shader))| (name.clone(), shader))
            .collect()
    }
    pub fn get_all(&self) -> Vec<(String, bool, &T)> {
        self.shaders
            .iter()
            .map(|(name, (enabled, shader))| (name.clone(), *enabled, shader))
            .collect()
    }
}

impl<T: ShaderProgram> Default for ShaderProgramMap<T> {
    fn default() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }
}

impl<T: ShaderProgram> std::ops::Index<&str> for ShaderProgramMap<T> {
    type Output = T;

    fn index(&self, index: &str) -> &T {
        self.get_shader(index).expect("Shader not found")
    }
}

impl<T: ShaderProgram> std::ops::IndexMut<&str> for ShaderProgramMap<T> {
    fn index_mut(&mut self, index: &str) -> &mut T {
        &mut self.shaders.get_mut(index).expect("Shader not found").1
    }
}
