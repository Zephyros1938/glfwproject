use std::collections::HashMap;

use log::debug;

use super::shader_program::ShaderProgram;

pub struct ShaderProgramMap {
    shaders: HashMap<String, (bool, ShaderProgram)>,
}

impl ShaderProgramMap {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: String, enabled: bool, shader: ShaderProgram) {
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

    pub fn get_shader(&self, name: &str) -> Option<&ShaderProgram> {
        self.shaders.get(name).map(|(_, shader)| shader)
    }
    pub fn get_shader_mut(&mut self, name: &str) -> Option<&mut ShaderProgram> {
        self.shaders.get_mut(name).map(|(_, shader)| shader)
    }
    pub fn get_all_1(&self) -> Vec<(String, &ShaderProgram)> {
        self.shaders
            .iter()
            .filter(|(_, (enabled, _))| *enabled)
            .map(|(name, (_, shader))| (name.clone(), shader))
            .collect()
    }
    pub fn get_all_0(&self) -> Vec<(String, &ShaderProgram)> {
        self.shaders
            .iter()
            .filter(|(_, (enabled, _))| !*enabled)
            .map(|(name, (_, shader))| (name.clone(), shader))
            .collect()
    }
    pub fn get_all(&self) -> Vec<(String, bool, &ShaderProgram)> {
        self.shaders
            .iter()
            .map(|(name, (enabled, shader))| (name.clone(), *enabled, shader))
            .collect()
    }
}

impl Default for ShaderProgramMap {
    fn default() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }
}

impl std::ops::Index<&str> for ShaderProgramMap {
    type Output = ShaderProgram;

    fn index(&self, index: &str) -> &ShaderProgram {
        self.get_shader(index).expect("Shader not found")
    }
}

impl std::ops::IndexMut<&str> for ShaderProgramMap {
    fn index_mut(&mut self, index: &str) -> &mut ShaderProgram {
        &mut self.shaders.get_mut(index).expect("Shader not found").1
    }
}
