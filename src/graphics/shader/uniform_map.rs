use log::{debug, error};
pub struct UniformMap {
    uniforms: std::collections::HashMap<String, Box<dyn std::any::Any>>,
}

impl UniformMap {
    /// Creates a new `UniformMap` instance.
    pub fn new() -> Self {
        debug!("Creating a new UniformMap");
        Self {
            uniforms: std::collections::HashMap::new(),
        }
    }

    /// Inserts a new uniform into the map.
    ///
    /// If a uniform with the same name already exists, returns an error.
    /// Otherwise, stores the data in a type-erased box.
    pub fn insert<T: 'static>(&mut self, name: impl Into<String>, data: T) -> Result<(), String> {
        let key = name.into();
        match self.uniforms.entry(key.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => {
                error!("Uniform '{}' already exists, insertion failed", key);
                Err(format!("Uniform '{}' already exists!", key))
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(Box::new(data));
                debug!("Uniform '{}' inserted successfully", key);
                Ok(())
            }
        }
    }

    /// Retrieves a reference to the uniform stored under `name` if it is of type `T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let value: Option<&i32> = uniform_map.get("my_uniform");
    /// ```
    pub fn get<T: 'static>(&self, name: &str) -> Option<&T> {
        self.uniforms
            .get(name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Retrieves a mutable reference to the uniform stored under `name` if it is of type `T`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let value_mut: Option<&mut i32> = uniform_map.get_mut("my_uniform");
    /// ```
    pub fn get_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        self.uniforms
            .get_mut(name)
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Checks whether a uniform with the given name exists.
    ///
    /// Returns `true` if the uniform exists, `false` otherwise.
    pub fn contains(&self, name: &str) -> bool {
        self.uniforms.contains_key(name)
    }

    /// Removes a uniform from the map by its name.
    ///
    /// Returns the removed uniform as a `Box<dyn Any>`, or `None` if it did not exist.
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn std::any::Any>> {
        if self.uniforms.contains_key(name) {
            debug!("Removing uniform '{}'", name);
            self.uniforms.remove(name)
        } else {
            debug!("Uniform '{}' not found for removal", name);
            None
        }
    }

    /// Clears all uniforms from the map.
    ///
    /// Useful for resetting the map to an empty state.
    pub fn clear(&mut self) {
        debug!("Clearing all uniforms");
        self.uniforms.clear();
    }
}
