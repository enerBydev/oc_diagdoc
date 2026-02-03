//! Comando init - Inicialización de proyectos.
//!
//! Crea la estructura inicial de un proyecto de documentación.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// INIT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Preset de inicialización.
#[derive(Debug, Clone, PartialEq)]
pub enum InitPreset {
    Minimal,
    Standard,
    Full,
    Custom,
}

/// Resultado de inicialización.
#[derive(Debug, Clone, Serialize)]
pub struct InitResult {
    pub project_path: PathBuf,
    pub files_created: Vec<PathBuf>,
    pub directories_created: Vec<PathBuf>,
}

impl InitResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            project_path: path,
            files_created: Vec::new(),
            directories_created: Vec::new(),
        }
    }

    pub fn total_items(&self) -> usize {
        self.files_created.len() + self.directories_created.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INIT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de inicialización.
#[derive(Parser, Debug, Clone)]
#[command(name = "init", about = "Inicializar proyecto")]
pub struct InitCommand {
    /// Ruta del proyecto.
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Preset: minimal, standard, full.
    #[arg(short, long, default_value = "standard")]
    pub preset: String,

    /// Forzar si ya existe.
    #[arg(short, long)]
    pub force: bool,

    // L20.1: Template de proyecto
    /// Template: proyecto o modulo.
    #[arg(long, short = 't', default_value = "proyecto")]
    pub template: String,

    /// Nombre del proyecto/módulo.
    #[arg(long)]
    pub name: Option<String>,

    // AN-08 FIX: Dry-run mode
    /// Modo preview: muestra qué se crearía sin ejecutar.
    #[arg(long)]
    pub dry_run: bool,
}

impl InitCommand {
    pub fn run(&self) -> OcResult<InitResult> {
        let mut result = InitResult::new(self.path.clone());

        // AN-08 FIX: Dry-run mode - muestra preview sin ejecutar
        if self.dry_run {
            eprintln!("🔍 [dry-run] Se crearía en: {}", self.path.display());
            eprintln!("  📁 {}/Datos/", self.path.display());
            eprintln!("  📄 {}/Datos/0. Contexualizador.md", self.path.display());
            eprintln!("  📄 {}/oc_diagdoc.yaml", self.path.display());
            if self.preset == "full" {
                eprintln!("  📁 {}/templates/", self.path.display());
                eprintln!("  📁 {}/snapshots/", self.path.display());
            }
            return Ok(result);
        }

        // Verificar si ya existe
        if self.path.join("oc_diagdoc.yaml").exists() && !self.force {
            return Err(crate::errors::OcError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Proyecto ya existe. Usa --force para sobrescribir.",
            )));
        }

        // L19.1: Crear estructura base
        let dirs = match self.template.as_str() {
            "modulo" => self.create_module_structure()?,
            _ => self.create_project_structure()?,
        };

        for dir in dirs {
            std::fs::create_dir_all(&dir)?;
            result.directories_created.push(dir);
        }

        // L19.2: Generar archivos base
        let files = self.generate_base_files()?;
        for (path, content) in files {
            std::fs::write(&path, content)?;
            result.files_created.push(path);
        }

        eprintln!("✅ Proyecto inicializado en: {}", self.path.display());
        Ok(result)
    }

    /// L19.1: Crea estructura de directorios para proyecto.
    fn create_project_structure(&self) -> OcResult<Vec<PathBuf>> {
        let preset = self.preset_enum();
        let base = &self.path;

        let mut dirs = vec![base.join("docs"), base.join("templates")];

        if matches!(preset, InitPreset::Standard | InitPreset::Full) {
            dirs.push(base.join("docs/modulos"));
            dirs.push(base.join("assets"));
        }

        if matches!(preset, InitPreset::Full) {
            dirs.push(base.join("docs/drafts"));
            dirs.push(base.join("docs/archive"));
            dirs.push(base.join("scripts"));
        }

        Ok(dirs)
    }

    /// L20.1: Crea estructura para módulo nuevo.
    fn create_module_structure(&self) -> OcResult<Vec<PathBuf>> {
        let module_name = self.name.as_deref().unwrap_or("nuevo_modulo");
        let base = self.path.join(format!("docs/modulos/{}", module_name));

        Ok(vec![base.clone(), base.join("assets")])
    }

    /// L19.2: Genera archivos base según template.
    fn generate_base_files(&self) -> OcResult<Vec<(PathBuf, String)>> {
        let mut files = Vec::new();
        let project_name = self.name.as_deref().unwrap_or(
            self.path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("MiProyecto"),
        );

        let now = chrono::Utc::now().format("%Y-%m-%d").to_string();

        match self.template.as_str() {
            "modulo" => {
                // Template para módulo
                let module_name = self.name.as_deref().unwrap_or("nuevo_modulo");
                let base = self.path.join(format!("docs/modulos/{}", module_name));

                let readme = format!(
                    r#"---
title: "{}"
document_id: "{}.0"
type: "module_root"
status: "draft"
created: "{}"
last_updated: "{}"
---

# {}

## Descripción

[Descripción del módulo]

## Documentos

| ID | Título | Estado |
|----|--------|--------|
| - | - | - |
"#,
                    module_name, module_name, now, now, module_name
                );

                files.push((base.join("README.md"), readme));
            }
            _ => {
                // Template para proyecto
                let config = format!(
                    r#"# Configuración oc_diagdoc
# Generado automáticamente

project:
  name: "{}"
  version: "1.0.0"
  created: "{}"

paths:
  docs: "docs"
  templates: "templates"
  assets: "assets"

validation:
  require_frontmatter: true
  require_document_id: true
  max_orphan_depth: 2

lint:
  enabled_rules:
    - L001  # Frontmatter
    - L002  # document_id
    - L003  # Title heading
    - L004  # Empty files
    - L005  # Broken links
"#,
                    project_name, now
                );

                files.push((self.path.join("oc_diagdoc.yaml"), config));

                let contextualizador = format!(
                    r#"---
title: "Contextualizador - {}"
document_id: "0"
type: "master"
status: "draft"
created: "{}"
last_updated: "{}"
children_count: 0
---

# {} - Documentación

## Descripción General

[Descripción del proyecto]

## Estructura de Módulos

| # | Módulo | Descripción |
|---|--------|-------------|
| 1 | [Módulo 1](./modulos/1/) | Por definir |

## Navegación

- [Inicio](#)
- [Módulos](./modulos/)
"#,
                    project_name, now, now, project_name
                );

                files.push((
                    self.path.join("docs/0. Contextualizador.md"),
                    contextualizador,
                ));

                // Template base
                let template = r#"---
title: "{{TITLE}}"
document_id: "{{ID}}"
parent_id: {{PARENT}}
status: "draft"
created: "{{CREATED}}"
last_updated: "{{UPDATED}}"
---

# {{TITLE}}

## Contenido

[Contenido del documento]
"#;
                files.push((
                    self.path.join("templates/document.md"),
                    template.to_string(),
                ));
            }
        }

        Ok(files)
    }

    pub fn preset_enum(&self) -> InitPreset {
        match self.preset.to_lowercase().as_str() {
            "minimal" | "min" => InitPreset::Minimal,
            "standard" | "std" => InitPreset::Standard,
            "full" => InitPreset::Full,
            _ => InitPreset::Custom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_result_new() {
        let result = InitResult::new(PathBuf::from("."));
        assert_eq!(result.total_items(), 0);
    }

    #[test]
    fn test_init_result_total() {
        let mut result = InitResult::new(PathBuf::from("."));
        result.files_created.push(PathBuf::from("a"));
        result.directories_created.push(PathBuf::from("b"));
        assert_eq!(result.total_items(), 2);
    }

    #[test]
    fn test_preset_enum() {
        let cmd = InitCommand {
            path: PathBuf::from("."),
            preset: "minimal".to_string(),
            force: false,
            template: "proyecto".to_string(),
            name: None,
            dry_run: false,
        };
        assert_eq!(cmd.preset_enum(), InitPreset::Minimal);
    }

    #[test]
    fn test_init_command_preset() {
        let cmd = InitCommand {
            path: PathBuf::from("."),
            preset: "full".to_string(),
            force: true,
            template: "proyecto".to_string(),
            name: Some("TestProject".to_string()),
            dry_run: false,
        };
        assert_eq!(cmd.preset_enum(), InitPreset::Full);
        assert!(cmd.force);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: InitCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;

    println!(
        "🚀 Inicializando proyecto en: {}",
        result.project_path.display()
    );
    println!(
        "📁 {} directorios creados",
        result.directories_created.len()
    );
    println!("📄 {} archivos creados", result.files_created.len());

    Ok(())
}
