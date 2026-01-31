//! Loader de proyectos.
//!
//! Integra FileScanner + YamlParser para cargar proyectos completos.

use std::path::Path;
use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
use crate::data::document::Document;
use crate::data::project::ProjectState;
use crate::core::config::OcConfig;
use crate::errors::{OcError, OcResult};

/// Carga un proyecto completo desde un directorio.
pub fn load_project(data_dir: impl AsRef<Path>) -> OcResult<ProjectState> {
    let data_dir = data_dir.as_ref();
    
    if !data_dir.exists() {
        return Err(OcError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Data directory not found: {}", data_dir.display())
        )));
    }
    
    // Configuración
    let mut config = OcConfig::default();
    config.data_dir = data_dir.to_path_buf();
    
    // Escanear archivos
    let options = ScanOptions::new();
    let files = get_all_md_files(data_dir, &options)?;
    
    // Parsear documentos usando Document::from_file
    let mut documents = Vec::new();
    for file_path in files {
        match Document::from_file(&file_path) {
            Ok(doc) => documents.push(doc),
            Err(e) => {
                // Log warning pero continuar
                eprintln!("Warning: Error loading {}: {}", file_path.display(), e);
            }
        }
    }
    
    // Crear estado
    let mut state = ProjectState::new(config);
    state.load_documents(documents);
    
    Ok(state)
}

/// Calcula estadísticas rápidas sin cargar todo el proyecto.
pub fn quick_stats(data_dir: impl AsRef<Path>) -> OcResult<QuickStats> {
    let data_dir = data_dir.as_ref();
    let options = ScanOptions::new();
    let files = get_all_md_files(data_dir, &options)?;
    
    let mut stats = QuickStats::default();
    stats.file_count = files.len();
    
    for file_path in &files {
        if let Ok(content) = read_file_content(file_path) {
            stats.total_words += content.split_whitespace().count();
            stats.total_bytes += content.len();
            
            // Detectar frontmatter válido
            if content.starts_with("---") {
                stats.with_frontmatter += 1;
            }
        }
    }
    
    Ok(stats)
}

#[derive(Debug, Clone, Default)]
pub struct QuickStats {
    pub file_count: usize,
    pub total_words: usize,
    pub total_bytes: usize,
    pub with_frontmatter: usize,
}

impl QuickStats {
    pub fn avg_words_per_file(&self) -> usize {
        if self.file_count == 0 { 0 } else { self.total_words / self.file_count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_doc(dir: &Path, name: &str, id: &str) {
        let content = format!(r#"---
id: "{}"
title: "Test Doc {}"
status: "borrador"
doc_type: "documento"
---

# Test Content

Some words here.
"#, id, id);
        fs::write(dir.join(name), content).unwrap();
    }

    #[test]
    fn test_load_project_empty() {
        let temp = TempDir::new().unwrap();
        let result = load_project(temp.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_quick_stats() {
        let temp = TempDir::new().unwrap();
        create_test_doc(temp.path(), "1.md", "1");
        create_test_doc(temp.path(), "2.md", "2");
        
        let stats = quick_stats(temp.path()).unwrap();
        assert_eq!(stats.file_count, 2);
        assert!(stats.total_words > 0);
    }
}
