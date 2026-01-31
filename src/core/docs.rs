//! Generador de documentación automática.
//!
//! Genera documentación del proyecto y API.

use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// DOCS TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Sección de documentación.
#[derive(Debug, Clone)]
pub struct DocSection {
    pub title: String,
    pub content: String,
    pub level: u8,
}

impl DocSection {
    pub fn new(title: &str, content: &str, level: u8) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            level,
        }
    }
    
    pub fn to_markdown(&self) -> String {
        let prefix = "#".repeat(self.level as usize);
        format!("{} {}\n\n{}", prefix, self.title, self.content)
    }
}

/// Documento generado.
#[derive(Debug, Clone)]
pub struct GeneratedDoc {
    pub name: String,
    pub sections: Vec<DocSection>,
}

impl GeneratedDoc {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sections: Vec::new(),
        }
    }
    
    pub fn add_section(&mut self, section: DocSection) {
        self.sections.push(section);
    }
    
    pub fn to_markdown(&self) -> String {
        self.sections.iter()
            .map(|s| s.to_markdown())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

/// Generador de README.
pub struct ReadmeGenerator {
    project_name: String,
    version: String,
}

impl ReadmeGenerator {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            project_name: name.to_string(),
            version: version.to_string(),
        }
    }
    
    pub fn generate(&self) -> GeneratedDoc {
        let mut doc = GeneratedDoc::new("README.md");
        
        doc.add_section(DocSection::new(
            &format!("{} v{}", self.project_name, self.version),
            "Sistema de diagnóstico de documentación técnica.",
            1,
        ));
        
        doc.add_section(DocSection::new(
            "Instalación",
            "```bash\ncargo install oc_diagdoc\n```",
            2,
        ));
        
        doc.add_section(DocSection::new(
            "Uso",
            "```bash\noc_diagdoc verify\noc_diagdoc stats\noc_diagdoc health\n```",
            2,
        ));
        
        doc
    }
}

/// Generador de CHANGELOG.
pub struct ChangelogGenerator {
    entries: Vec<ChangelogEntry>,
}

#[derive(Debug, Clone)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: String,
    pub changes: Vec<String>,
}

impl ChangelogGenerator {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }
    
    pub fn add_entry(&mut self, entry: ChangelogEntry) {
        self.entries.push(entry);
    }
    
    pub fn generate(&self) -> GeneratedDoc {
        let mut doc = GeneratedDoc::new("CHANGELOG.md");
        
        doc.add_section(DocSection::new("Changelog", "Historial de cambios.", 1));
        
        for entry in &self.entries {
            let content = entry.changes.iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n");
            doc.add_section(DocSection::new(
                &format!("[{}] - {}", entry.version, entry.date),
                &content,
                2,
            ));
        }
        
        doc
    }
}

impl Default for ChangelogGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_section_new() {
        let section = DocSection::new("Title", "Content", 2);
        assert_eq!(section.level, 2);
    }

    #[test]
    fn test_doc_section_markdown() {
        let section = DocSection::new("Title", "Content", 2);
        assert!(section.to_markdown().starts_with("## Title"));
    }

    #[test]
    fn test_readme_generator() {
        let gen = ReadmeGenerator::new("test", "1.0");
        let doc = gen.generate();
        assert!(!doc.sections.is_empty());
    }

    #[test]
    fn test_changelog_generator() {
        let mut gen = ChangelogGenerator::new();
        gen.add_entry(ChangelogEntry {
            version: "1.0.0".to_string(),
            date: "2024-01-01".to_string(),
            changes: vec!["Initial release".to_string()],
        });
        let doc = gen.generate();
        assert!(doc.to_markdown().contains("1.0.0"));
    }
}
