//! Comando gen - Generación de documentos.
//!
//! Genera nuevos documentos a partir de templates.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// GEN TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de documento a generar.
#[derive(Debug, Clone, PartialEq)]
pub enum DocType {
    Module,
    Document,
    Index,
    Readme,
    Custom(String),
}

impl DocType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "module" => Self::Module,
            "document" | "doc" => Self::Document,
            "index" => Self::Index,
            "readme" => Self::Readme,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// Resultado de generación.
#[derive(Debug, Clone, Serialize)]
pub struct GenResult {
    pub created_files: Vec<PathBuf>,
    pub template_used: String,
    pub variables_applied: usize,
}

impl GenResult {
    pub fn new(template: &str) -> Self {
        Self {
            created_files: Vec::new(),
            template_used: template.to_string(),
            variables_applied: 0,
        }
    }
    
    pub fn add_file(&mut self, path: PathBuf) {
        self.created_files.push(path);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GEN COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de generación.
#[derive(Parser, Debug, Clone)]
#[command(name = "gen", about = "Generar documentos")]
pub struct GenCommand {
    /// Tipo de documento.
    pub doc_type: String,
    
    /// ID del documento.
    pub doc_id: String,
    
    /// Ruta de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Template a usar.
    #[arg(short, long)]
    pub template: Option<String>,
    
    /// Título del documento.
    #[arg(long)]
    pub title: Option<String>,
}

impl GenCommand {
    pub fn run(&self) -> OcResult<GenResult> {
        let template = self.template.as_deref().unwrap_or("default");
        let result = GenResult::new(template);
        // TODO: Implementar generación real
        Ok(result)
    }
    
    pub fn doc_type(&self) -> DocType {
        DocType::from_str(&self.doc_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_result_new() {
        let result = GenResult::new("default");
        assert_eq!(result.template_used, "default");
    }

    #[test]
    fn test_add_file() {
        let mut result = GenResult::new("test");
        result.add_file(PathBuf::from("new.md"));
        assert_eq!(result.created_files.len(), 1);
    }

    #[test]
    fn test_doc_type_from_str() {
        assert_eq!(DocType::from_str("module"), DocType::Module);
        assert_eq!(DocType::from_str("doc"), DocType::Document);
    }

    #[test]
    fn test_doc_type_custom() {
        match DocType::from_str("special") {
            DocType::Custom(s) => assert_eq!(s, "special"),
            _ => panic!("Expected Custom variant"),
        }
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: GenCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("📝 Generando {:?} con ID: {}", cmd.doc_type(), cmd.doc_id);
    println!("📋 Template: {}", result.template_used);
    
    for file in &result.created_files {
        println!("  ✅ {}", file.display());
    }
    
    Ok(())
}
