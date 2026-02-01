//! Comando gen - Generación de documentos.
//!
//! Genera nuevos documentos a partir de templates.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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

    /// ID del documento (o 'auto' para generar).
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

    // L13-L14: Flags avanzados
    /// Parent ID para calcular jerarquía.
    #[arg(long)]
    pub parent_id: Option<String>,

    /// Módulo al que pertenece.
    #[arg(long, short = 'm')]
    pub module: Option<String>,

    /// Variables adicionales (formato: VAR=VALUE).
    #[arg(long)]
    pub var: Option<Vec<String>>,

    /// Validar estructura después de generar.
    #[arg(long)]
    pub validate: bool,
}

/// L14.1: Variables para templates.
#[derive(Debug, Clone)]
pub struct TemplateVars {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub module: Option<String>,
    pub created: String,
    pub updated: String,
    pub custom: std::collections::HashMap<String, String>,
}

impl GenCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<GenResult> {
        let template = self.template.as_deref().unwrap_or("default");
        let mut result = GenResult::new(template);

        // L13.2: Auto-generar ID si se pide
        let doc_id = if self.doc_id == "auto" {
            self.auto_generate_id(data_dir)?
        } else {
            self.doc_id.clone()
        };

        // L13.3: Auto-calcular parent_id si no se da
        let parent_id = self
            .parent_id
            .clone()
            .or_else(|| self.calculate_parent_id(&doc_id));

        // L13.4 + L14.1: Preparar variables
        let now = chrono::Utc::now();
        let vars = TemplateVars {
            id: doc_id.clone(),
            title: self
                .title
                .clone()
                .unwrap_or_else(|| format!("Documento {}", doc_id)),
            parent_id: parent_id.clone(),
            module: self.module.clone(),
            created: now.format("%Y-%m-%d").to_string(),
            updated: now.format("%Y-%m-%d").to_string(),
            custom: self.parse_custom_vars(),
        };

        // Generar contenido desde template
        let content = self.render_template(&vars);
        result.variables_applied = 6 + vars.custom.len();

        // Determinar output path
        let output_path = self
            .output
            .clone()
            .unwrap_or_else(|| data_dir.join(format!("{}.md", doc_id)));

        // Escribir archivo
        std::fs::write(&output_path, &content)?;
        result.add_file(output_path.clone());

        // L14.3: Validar si se pidió
        if self.validate {
            self.validate_generated(&output_path)?;
        }

        eprintln!("✅ Generado: {} (ID: {})", output_path.display(), doc_id);
        Ok(result)
    }

    /// L13.2: Auto-genera un ID único basado en módulo y conteo.
    fn auto_generate_id(&self, data_dir: &std::path::Path) -> OcResult<String> {
        use crate::core::files::{get_all_md_files, ScanOptions};

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let prefix = self.module.as_deref().unwrap_or("0");
        let count = files
            .iter()
            .filter(|f| {
                f.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.starts_with(prefix))
                    .unwrap_or(false)
            })
            .count();

        Ok(format!("{}.{}", prefix, count + 1))
    }

    /// L13.3: Calcula parent_id basado en el ID del documento.
    fn calculate_parent_id(&self, doc_id: &str) -> Option<String> {
        let parts: Vec<&str> = doc_id.split('.').collect();
        if parts.len() > 1 {
            // "1.2.3" -> "1.2"
            Some(parts[..parts.len() - 1].join("."))
        } else {
            None
        }
    }

    /// L14.1: Parsea variables custom de --var.
    fn parse_custom_vars(&self) -> std::collections::HashMap<String, String> {
        let mut vars = std::collections::HashMap::new();
        if let Some(ref var_list) = self.var {
            for v in var_list {
                if let Some((key, value)) = v.split_once('=') {
                    vars.insert(key.to_string(), value.to_string());
                }
            }
        }
        vars
    }

    /// L13.1: Renderiza template con variables.
    fn render_template(&self, vars: &TemplateVars) -> String {
        let doc_type = self.doc_type();

        let base_template = match doc_type {
            DocType::Module => {
                r#"---
title: "{{TITLE}}"
document_id: "{{ID}}"
parent_id: {{PARENT}}
module: "{{MODULE}}"
status: "draft"
created: "{{CREATED}}"
last_updated: "{{UPDATED}}"
---

# {{TITLE}}

## Descripción

[Descripción del módulo]

## Contenido

[Contenido principal]
"#
            }
            DocType::Document => {
                r#"---
title: "{{TITLE}}"
document_id: "{{ID}}"
parent_id: {{PARENT}}
module: "{{MODULE}}"
status: "draft"
created: "{{CREATED}}"
last_updated: "{{UPDATED}}"
---

# {{TITLE}}

## Introducción

[Introducción del documento]

## Contenido

[Contenido principal]

## Referencias

- [Referencias relevantes]
"#
            }
            DocType::Index => {
                r#"---
title: "Índice - {{TITLE}}"
document_id: "{{ID}}"
type: "index"
created: "{{CREATED}}"
---

# Índice: {{TITLE}}

## Documentos

| ID | Título | Estado |
|----|--------|--------|
| - | - | - |
"#
            }
            _ => {
                r#"---
title: "{{TITLE}}"
document_id: "{{ID}}"
parent_id: {{PARENT}}
created: "{{CREATED}}"
last_updated: "{{UPDATED}}"
---

# {{TITLE}}

[Contenido]
"#
            }
        };

        let parent_value = vars
            .parent_id
            .as_ref()
            .map(|p| format!("\"{}\"", p))
            .unwrap_or_else(|| "null".to_string());

        let module_value = vars
            .module
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "General".to_string());

        let mut content = base_template
            .to_string()
            .replace("{{TITLE}}", &vars.title)
            .replace("{{ID}}", &vars.id)
            .replace("{{PARENT}}", &parent_value)
            .replace("{{MODULE}}", &module_value)
            .replace("{{CREATED}}", &vars.created)
            .replace("{{UPDATED}}", &vars.updated);

        // L14.1: Aplicar variables custom
        for (key, value) in &vars.custom {
            content = content.replace(&format!("{{{{{}}}}}", key.to_uppercase()), value);
        }

        content
    }

    /// L14.3: Valida estructura del documento generado.
    fn validate_generated(&self, path: &PathBuf) -> OcResult<()> {
        let content = std::fs::read_to_string(path)?;

        // Verificar frontmatter
        if !content.starts_with("---") {
            eprintln!("⚠️ Advertencia: documento sin frontmatter válido");
        }

        // Verificar campos requeridos
        let required = ["title:", "document_id:"];
        for field in required {
            if !content.contains(field) {
                eprintln!("⚠️ Campo faltante: {}", field);
            }
        }

        Ok(())
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
pub fn run(cmd: GenCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd
        .output
        .as_ref()
        .and_then(|o| o.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or(default_dir);
    let result = cmd.run(&data_dir)?;

    println!("📝 Generando {:?} con ID: {}", cmd.doc_type(), cmd.doc_id);
    println!("📋 Template: {}", result.template_used);
    println!("📊 {} variables aplicadas", result.variables_applied);

    for file in &result.created_files {
        println!("  ✅ {}", file.display());
    }

    Ok(())
}
