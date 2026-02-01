//! Comando template - Gestión de templates.
//!
//! Lista, crea y administra templates de documentos.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// TEMPLATE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Información de un template.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub name: String,
    pub path: PathBuf,
    pub variables: Vec<String>,
    pub description: String,
}

impl TemplateInfo {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
            variables: Vec::new(),
            description: String::new(),
        }
    }
}

/// Resultado de operación de template.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateResult {
    pub templates: Vec<TemplateInfo>,
    pub action: String,
}

impl TemplateResult {
    pub fn list(templates: Vec<TemplateInfo>) -> Self {
        Self {
            templates,
            action: "list".to_string(),
        }
    }

    pub fn created(template: TemplateInfo) -> Self {
        Self {
            templates: vec![template],
            action: "created".to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TEMPLATE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de templates.
#[derive(Parser, Debug, Clone)]
#[command(name = "template", about = "Gestión de templates")]
pub struct TemplateCommand {
    /// Nombre del template.
    pub name: Option<String>,

    /// Listar templates.
    #[arg(short, long)]
    pub list: bool,

    /// Crear nuevo template.
    #[arg(short, long)]
    pub create: bool,

    /// Ruta del template.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

impl TemplateCommand {
    pub fn run(&self) -> OcResult<TemplateResult> {
        if self.list || self.name.is_none() {
            // Listar templates disponibles
            let templates = vec![
                TemplateInfo::new("document", PathBuf::from("templates/document.md")),
                TemplateInfo::new("module", PathBuf::from("templates/module.md")),
                TemplateInfo::new("index", PathBuf::from("templates/index.md")),
            ];
            Ok(TemplateResult::list(templates))
        } else {
            // Crear template
            let info = TemplateInfo::new(
                self.name.as_deref().unwrap_or("new"),
                self.path
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("templates/new.md")),
            );
            Ok(TemplateResult::created(info))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_info_new() {
        let info = TemplateInfo::new("test", PathBuf::from("templates/test.md"));
        assert_eq!(info.name, "test");
    }

    #[test]
    fn test_template_result_list() {
        let result = TemplateResult::list(vec![]);
        assert_eq!(result.action, "list");
    }

    #[test]
    fn test_template_result_created() {
        let info = TemplateInfo::new("new", PathBuf::from("t.md"));
        let result = TemplateResult::created(info);
        assert_eq!(result.action, "created");
    }

    #[test]
    fn test_template_command_list() {
        let cmd = TemplateCommand {
            name: None,
            list: true,
            create: false,
            path: None,
        };
        let result = cmd.run().unwrap();
        assert!(!result.templates.is_empty());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: TemplateCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;

    if result.action == "list" {
        println!("📋 Templates disponibles:\n");
        for t in &result.templates {
            println!("  📄 {} - {}", t.name, t.path.display());
        }
    } else {
        println!("✅ Template creado: {}", result.templates[0].name);
    }

    Ok(())
}
