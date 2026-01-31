//! Modelo de módulo de documentación.
//!
//! Agrupa documentos por módulo jerárquico.

use std::collections::HashMap;
use crate::types::DocumentId;
use crate::data::document::Document;

// ═══════════════════════════════════════════════════════════════════════════
// MODULE STATS
// ═══════════════════════════════════════════════════════════════════════════

/// Estadísticas de un módulo.
#[derive(Debug, Clone, Default)]
pub struct ModuleStats {
    /// Número de documentos.
    pub doc_count: usize,
    /// Total de palabras.
    pub word_count: usize,
    /// Documentos saludables.
    pub healthy_count: usize,
    /// Cobertura (porcentaje de documentos saludables).
    pub coverage: f64,
    /// Profundidad máxima.
    pub max_depth: usize,
}

impl ModuleStats {
    pub fn compute(docs: &[&Document]) -> Self {
        let doc_count = docs.len();
        let word_count: usize = docs.iter().map(|d| d.word_count).sum();
        let healthy_count = docs.iter().filter(|d| d.is_healthy()).count();
        let coverage = if doc_count > 0 {
            (healthy_count as f64 / doc_count as f64) * 100.0
        } else {
            0.0
        };
        let max_depth = docs.iter().map(|d| d.depth()).max().unwrap_or(0);
        
        Self {
            doc_count,
            word_count,
            healthy_count,
            coverage,
            max_depth,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE
// ═══════════════════════════════════════════════════════════════════════════

/// Un módulo de documentación.
#[derive(Debug, Clone)]
pub struct Module {
    /// ID del módulo (1, 2, 3, etc.)
    pub id: u32,
    /// Nombre del módulo.
    pub name: String,
    /// Documento raíz del módulo.
    pub root_doc: Option<DocumentId>,
    /// IDs de todos los documentos en el módulo.
    pub doc_ids: Vec<DocumentId>,
    /// Estadísticas.
    pub stats: ModuleStats,
}

impl Module {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            root_doc: None,
            doc_ids: Vec::new(),
            stats: ModuleStats::default(),
        }
    }
    
    /// Número de documentos.
    pub fn doc_count(&self) -> usize {
        self.doc_ids.len()
    }
    
    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.doc_ids.is_empty()
    }
    
    /// Cobertura del módulo.
    pub fn coverage(&self) -> f64 {
        self.stats.coverage
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Registro de módulos.
#[derive(Debug, Default)]
pub struct ModuleRegistry {
    modules: HashMap<u32, Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Construye desde documentos.
    pub fn from_documents(docs: &[Document]) -> Self {
        let mut registry = Self::new();
        
        // Agrupar documentos por módulo
        let mut by_module: HashMap<u32, Vec<&Document>> = HashMap::new();
        for doc in docs {
            if let Ok(module) = doc.module() {
                by_module.entry(module).or_default().push(doc);
            }
        }
        
        // Crear módulos
        for (id, module_docs) in by_module {
            let name = format!("Módulo {}", id);
            let mut module = Module::new(id, name);
            
            module.doc_ids = module_docs.iter()
                .filter_map(|d| d.id().ok())
                .collect();
            
            // Encontrar documento raíz
            module.root_doc = module_docs.iter()
                .filter_map(|d| d.id().ok())
                .find(|id| id.depth() == 1);
            
            module.stats = ModuleStats::compute(&module_docs);
            
            registry.modules.insert(id, module);
        }
        
        registry
    }
    
    /// Obtiene un módulo.
    pub fn get(&self, id: u32) -> Option<&Module> {
        self.modules.get(&id)
    }
    
    /// Número de módulos.
    pub fn len(&self) -> usize {
        self.modules.len()
    }
    
    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
    
    /// Iterador sobre módulos.
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Module)> {
        self.modules.iter()
    }
    
    /// Módulos ordenados por ID.
    pub fn sorted(&self) -> Vec<&Module> {
        let mut modules: Vec<&Module> = self.modules.values().collect();
        modules.sort_by_key(|m| m.id);
        modules
    }
    
    /// Total de documentos.
    pub fn total_docs(&self) -> usize {
        self.modules.values().map(|m| m.doc_count()).sum()
    }
    
    /// Total de palabras.
    pub fn total_words(&self) -> usize {
        self.modules.values().map(|m| m.stats.word_count).sum()
    }
    
    /// Cobertura promedio.
    pub fn average_coverage(&self) -> f64 {
        if self.modules.is_empty() {
            return 0.0;
        }
        let total: f64 = self.modules.values().map(|m| m.coverage()).sum();
        total / self.modules.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_new() {
        let module = Module::new(1, "Test Module");
        assert_eq!(module.id, 1);
        assert!(module.is_empty());
    }

    #[test]
    fn test_module_stats() {
        let stats = ModuleStats::default();
        assert_eq!(stats.doc_count, 0);
        assert_eq!(stats.coverage, 0.0);
    }

    #[test]
    fn test_module_registry() {
        let registry = ModuleRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_sorted_modules() {
        let mut registry = ModuleRegistry::new();
        registry.modules.insert(3, Module::new(3, "Module 3"));
        registry.modules.insert(1, Module::new(1, "Module 1"));
        registry.modules.insert(2, Module::new(2, "Module 2"));
        
        let sorted = registry.sorted();
        assert_eq!(sorted[0].id, 1);
        assert_eq!(sorted[1].id, 2);
        assert_eq!(sorted[2].id, 3);
    }
}
