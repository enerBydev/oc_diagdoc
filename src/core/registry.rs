//! Registry de comandos y plugins.
//!
//! Sistema de registro dinámico para extensibilidad.

use std::collections::HashMap;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Plugin ejecutable.
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, args: &[String]) -> Result<(), String>;
}

/// Metadata de un comando registrado.
#[derive(Debug, Clone)]
pub struct CommandMeta {
    pub name: String,
    pub description: String,
    pub category: String,
    pub aliases: Vec<String>,
}

impl CommandMeta {
    pub fn new(name: &str, description: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            aliases: Vec::new(),
        }
    }
    
    pub fn with_alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_string());
        self
    }
}

/// Registry de comandos.
#[derive(Default)]
pub struct CommandRegistry {
    commands: HashMap<String, CommandMeta>,
    plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn register(&mut self, meta: CommandMeta) {
        for alias in &meta.aliases {
            self.commands.insert(alias.clone(), meta.clone());
        }
        self.commands.insert(meta.name.clone(), meta);
    }
    
    pub fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, Arc::new(plugin));
    }
    
    pub fn get(&self, name: &str) -> Option<&CommandMeta> {
        self.commands.get(name)
    }
    
    pub fn get_plugin(&self, name: &str) -> Option<Arc<dyn Plugin>> {
        self.plugins.get(name).cloned()
    }
    
    pub fn list_commands(&self) -> Vec<&CommandMeta> {
        let mut seen = std::collections::HashSet::new();
        self.commands.values()
            .filter(|meta| seen.insert(&meta.name))
            .collect()
    }
    
    pub fn list_by_category(&self, category: &str) -> Vec<&CommandMeta> {
        self.list_commands()
            .into_iter()
            .filter(|meta| meta.category == category)
            .collect()
    }
    
    pub fn command_count(&self) -> usize {
        self.list_commands().len()
    }
    
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

/// Builder para el registry.
pub struct RegistryBuilder {
    registry: CommandRegistry,
}

impl RegistryBuilder {
    pub fn new() -> Self {
        Self { registry: CommandRegistry::new() }
    }
    
    pub fn with_builtins(mut self) -> Self {
        // Comandos analíticos
        self.registry.register(CommandMeta::new("verify", "Verificar documentación", "analytics"));
        self.registry.register(CommandMeta::new("stats", "Estadísticas", "analytics"));
        self.registry.register(CommandMeta::new("search", "Buscar contenido", "analytics"));
        self.registry.register(CommandMeta::new("deps", "Dependencias", "analytics"));
        self.registry.register(CommandMeta::new("tree", "Árbol jerárquico", "analytics"));
        
        // Comandos de modificación
        self.registry.register(CommandMeta::new("batch", "Operaciones en lote", "modification"));
        self.registry.register(CommandMeta::new("sync", "Sincronizar metadatos", "modification"));
        self.registry.register(CommandMeta::new("links", "Gestión de enlaces", "modification"));
        
        // Comandos de diagnóstico
        self.registry.register(CommandMeta::new("lint", "Análisis estático", "diagnostic"));
        self.registry.register(CommandMeta::new("health", "Salud del proyecto", "diagnostic"));
        self.registry.register(CommandMeta::new("coverage", "Cobertura de contenido", "diagnostic"));
        
        self
    }
    
    pub fn build(self) -> CommandRegistry {
        self.registry
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_meta_new() {
        let meta = CommandMeta::new("test", "Test command", "util");
        assert_eq!(meta.name, "test");
    }

    #[test]
    fn test_command_meta_alias() {
        let meta = CommandMeta::new("test", "Test", "util").with_alias("t");
        assert!(meta.aliases.contains(&"t".to_string()));
    }

    #[test]
    fn test_registry_register() {
        let mut registry = CommandRegistry::new();
        registry.register(CommandMeta::new("cmd", "Desc", "cat"));
        assert!(registry.get("cmd").is_some());
    }

    #[test]
    fn test_registry_builder() {
        let registry = RegistryBuilder::new().with_builtins().build();
        assert!(registry.command_count() > 0);
    }
}
