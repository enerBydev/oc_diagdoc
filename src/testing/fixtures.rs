//! Utilidades de testing.
//!
//! Fixtures, mocks y helpers para tests.

use std::path::PathBuf;
use tempfile::TempDir;

// ═══════════════════════════════════════════════════════════════════════════
// TEST FIXTURES
// ═══════════════════════════════════════════════════════════════════════════

/// Fixture para proyecto de test.
pub struct TestProject {
    pub temp_dir: TempDir,
    pub data_dir: PathBuf,
}

impl TestProject {
    pub fn new() -> std::io::Result<Self> {
        let temp_dir = TempDir::new()?;
        let data_dir = temp_dir.path().join("Datos");
        std::fs::create_dir_all(&data_dir)?;

        Ok(Self { temp_dir, data_dir })
    }

    pub fn create_module(&self, module_num: u32) -> std::io::Result<PathBuf> {
        let module_dir = self.data_dir.join(format!("Módulo {}", module_num));
        std::fs::create_dir_all(&module_dir)?;
        Ok(module_dir)
    }

    pub fn create_document(
        &self,
        module: u32,
        doc_id: &str,
        content: &str,
    ) -> std::io::Result<PathBuf> {
        let module_dir = self.create_module(module)?;
        let doc_path = module_dir.join(format!("{}.md", doc_id));
        std::fs::write(&doc_path, content)?;
        Ok(doc_path)
    }

    pub fn path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }
}

impl Default for TestProject {
    fn default() -> Self {
        Self::new().expect("Failed to create test project")
    }
}

/// Genera frontmatter válido.
pub fn sample_frontmatter(title: &str, status: &str) -> String {
    format!(
        r#"---
id: "1.1"
title: "{}"
status: "{}"
doc_type: "documento"
created: "2024-01-01"
last_updated: "2024-01-01"
---

# {}

Contenido de ejemplo.
"#,
        title, status, title
    )
}

/// Genera documento mínimo.
pub fn minimal_document(id: &str, title: &str) -> String {
    format!(
        r#"---
id: "{}"
title: "{}"
status: "en_progreso"
doc_type: "documento"
---

# {}
"#,
        id, title, title
    )
}

/// Genera múltiples documentos para un módulo.
pub fn generate_module_docs(
    project: &TestProject,
    module: u32,
    count: usize,
) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for i in 1..=count {
        let doc_id = format!("{}.{}", module, i);
        let content = minimal_document(&doc_id, &format!("Documento {}", i));
        let path = project.create_document(module, &doc_id, &content)?;
        paths.push(path);
    }
    Ok(paths)
}

// ═══════════════════════════════════════════════════════════════════════════
// ASSERTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Assert que un archivo existe.
pub fn assert_file_exists(path: &std::path::Path) {
    assert!(path.exists(), "File should exist: {}", path.display());
}

/// Assert que un archivo contiene texto.
pub fn assert_file_contains(path: &std::path::Path, text: &str) {
    let content =
        std::fs::read_to_string(path).expect(&format!("Failed to read file: {}", path.display()));
    assert!(
        content.contains(text),
        "File should contain '{}': {}",
        text,
        path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_project_new() {
        let project = TestProject::new().unwrap();
        assert!(project.data_dir.exists());
    }

    #[test]
    fn test_create_module() {
        let project = TestProject::new().unwrap();
        let module_path = project.create_module(1).unwrap();
        assert!(module_path.exists());
    }

    #[test]
    fn test_create_document() {
        let project = TestProject::new().unwrap();
        let doc_path = project.create_document(1, "1.1", "# Test").unwrap();
        assert!(doc_path.exists());
    }

    #[test]
    fn test_sample_frontmatter() {
        let fm = sample_frontmatter("Test Doc", "completado");
        assert!(fm.contains("title: \"Test Doc\""));
        assert!(fm.contains("status: \"completado\""));
    }

    #[test]
    fn test_generate_module_docs() {
        let project = TestProject::new().unwrap();
        let docs = generate_module_docs(&project, 1, 3).unwrap();
        assert_eq!(docs.len(), 3);
    }
}
