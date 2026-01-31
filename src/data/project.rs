//! Estado global del proyecto.
//!
//! Contiene toda la información sobre el proyecto de documentación.

use std::path::PathBuf;
use std::collections::HashMap;
use crate::types::DocumentId;
use crate::data::document::{Document, DocumentCollection};
use crate::data::hierarchy::HierarchyTree;
use crate::data::module::ModuleRegistry;
use crate::core::config::OcConfig;

// ═══════════════════════════════════════════════════════════════════════════
// PROJECT STATS
// ═══════════════════════════════════════════════════════════════════════════

/// Estadísticas globales del proyecto.
#[derive(Debug, Clone, Default)]
pub struct ProjectStats {
    /// Total de documentos.
    pub total_docs: usize,
    /// Total de palabras.
    pub total_words: usize,
    /// Documentos saludables.
    pub healthy_docs: usize,
    /// Documentos no saludables.
    pub unhealthy_docs: usize,
    /// Cobertura global.
    pub coverage: f64,
    /// Número de módulos.
    pub module_count: usize,
    /// Profundidad máxima.
    pub max_depth: usize,
    /// Número de enlaces.
    pub total_links: usize,
    /// Enlaces rotos.
    pub broken_links: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// PROJECT STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Estado completo del proyecto.
#[derive(Debug)]
pub struct ProjectState {
    /// Configuración.
    pub config: OcConfig,
    /// Directorio de datos.
    pub data_dir: PathBuf,
    /// Documentos cargados.
    pub documents: DocumentCollection,
    /// Árbol jerárquico.
    pub hierarchy: HierarchyTree,
    /// Registro de módulos.
    pub modules: ModuleRegistry,
    /// Estadísticas.
    pub stats: ProjectStats,
    /// Índice por ID.
    doc_index: HashMap<DocumentId, usize>,
}

impl ProjectState {
    /// Crea estado vacío.
    pub fn new(config: OcConfig) -> Self {
        let data_dir = config.data_dir.clone();
        Self {
            config,
            data_dir,
            documents: DocumentCollection::new(),
            hierarchy: HierarchyTree::new(),
            modules: ModuleRegistry::new(),
            stats: ProjectStats::default(),
            doc_index: HashMap::new(),
        }
    }
    
    /// Carga documentos desde el directorio de datos.
    pub fn load_documents(&mut self, docs: Vec<Document>) {
        // Indexar
        for (i, doc) in docs.iter().enumerate() {
            if let Ok(id) = doc.id() {
                self.doc_index.insert(id, i);
            }
        }
        
        // Construir estructuras
        self.hierarchy = HierarchyTree::from_documents(docs.clone());
        self.modules = ModuleRegistry::from_documents(&docs);
        self.documents = DocumentCollection::from(docs);
        
        // Calcular estadísticas
        self.compute_stats();
    }
    
    fn compute_stats(&mut self) {
        let docs: Vec<_> = self.documents.iter().collect();
        
        self.stats.total_docs = docs.len();
        self.stats.total_words = docs.iter().map(|d| d.word_count).sum();
        self.stats.healthy_docs = docs.iter().filter(|d| d.is_healthy()).count();
        self.stats.unhealthy_docs = self.stats.total_docs - self.stats.healthy_docs;
        self.stats.coverage = if self.stats.total_docs > 0 {
            (self.stats.healthy_docs as f64 / self.stats.total_docs as f64) * 100.0
        } else {
            0.0
        };
        self.stats.module_count = self.modules.len();
        self.stats.max_depth = self.hierarchy.max_depth();
        self.stats.total_links = docs.iter().map(|d| d.links.len()).sum();
        self.stats.broken_links = docs.iter().map(|d| d.broken_link_count()).sum();
    }
    
    /// Busca documento por ID.
    pub fn get_document(&self, id: &DocumentId) -> Option<&Document> {
        self.doc_index.get(id)
            .and_then(|&i| self.documents.iter().nth(i))
    }
    
    /// Número de documentos.
    pub fn doc_count(&self) -> usize {
        self.stats.total_docs
    }
    
    /// ¿Está el proyecto vacío?
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_state_new() {
        let config = OcConfig::default();
        let state = ProjectState::new(config);
        
        assert!(state.is_empty());
        assert_eq!(state.doc_count(), 0);
    }

    #[test]
    fn test_project_stats_default() {
        let stats = ProjectStats::default();
        assert_eq!(stats.total_docs, 0);
        assert_eq!(stats.coverage, 0.0);
    }
}
