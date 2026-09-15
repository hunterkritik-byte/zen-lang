//! Module system for Zen
//! Supports modular code organization with public/private visibility and namespacing

use std::collections::HashMap;
use std::fmt;

/// Represents a module in Zen
#[derive(Clone, Debug)]
pub struct ZenModule {
    /// Module name
    pub name: String,
    /// Public exports
    pub exports: HashMap<String, String>,
    /// Private definitions
    pub private: HashMap<String, String>,
    /// Sub-modules
    pub submodules: HashMap<String, ZenModule>,
}

impl ZenModule {
    /// Create a new module
    pub fn new(name: String) -> Self {
        ZenModule {
            name,
            exports: HashMap::new(),
            private: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    /// Export a definition
    pub fn export(&mut self, name: String, definition: String) {
        self.exports.insert(name, definition);
    }

    /// Define private definition
    pub fn define_private(&mut self, name: String, definition: String) {
        self.private.insert(name, definition);
    }

    /// Add sub-module
    pub fn add_submodule(&mut self, submodule: ZenModule) {
        self.submodules.insert(submodule.name.clone(), submodule);
    }

    /// Get exported definition
    pub fn get_export(&self, name: &str) -> Option<String> {
        self.exports.get(name).cloned()
    }

    /// Get sub-module
    pub fn get_submodule(&self, name: &str) -> Option<&ZenModule> {
        self.submodules.get(name)
    }

    /// Check if definition is exported
    pub fn is_exported(&self, name: &str) -> bool {
        self.exports.contains_key(name)
    }
}

impl fmt::Display for ZenModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module {}", self.name)?;
        if !self.exports.is_empty() {
            write!(f, " [exports: {}]", self.exports.len())?;
        }
        Ok(())
    }
}

/// Module loader for imports
#[derive(Debug)]
pub struct ModuleLoader {
    modules: HashMap<String, ZenModule>,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new() -> Self {
        ModuleLoader {
            modules: HashMap::new(),
        }
    }

    /// Register a module
    pub fn register(&mut self, module: ZenModule) {
        self.modules.insert(module.name.clone(), module);
    }

    /// Load a module by name
    pub fn load(&self, name: &str) -> Option<ZenModule> {
        self.modules.get(name).cloned()
    }

    /// Check if module exists
    pub fn exists(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = ZenModule::new("math".to_string());
        assert_eq!(module.name, "math");
    }

    #[test]
    fn test_module_export() {
        let mut module = ZenModule::new("math".to_string());
        module.export("add".to_string(), "fn add(a, b) { a + b }".to_string());
        assert!(module.is_exported("add"));
        assert!(module.get_export("add").is_some());
    }

    #[test]
    fn test_module_loader() {
        let mut loader = ModuleLoader::new();
        let module = ZenModule::new("math".to_string());
        loader.register(module);
        assert!(loader.exists("math"));
        assert!(loader.load("math").is_some());
    }
}
