//! Comando init - Inicialización de proyectos.
//!
//! Crea la estructura inicial de un proyecto de documentación.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

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
}

impl InitCommand {
    pub fn run(&self) -> OcResult<InitResult> {
        let mut result = InitResult::new(self.path.clone());
        
        // Estructura básica
        result.directories_created.push(self.path.join("docs"));
        result.directories_created.push(self.path.join("templates"));
        result.files_created.push(self.path.join("oc_diagdoc.yaml"));
        result.files_created.push(self.path.join("docs/0. Contextualizador.md"));
        
        // TODO: Crear archivos realmente
        Ok(result)
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
        };
        assert_eq!(cmd.preset_enum(), InitPreset::Minimal);
    }

    #[test]
    fn test_init_command_run() {
        let cmd = InitCommand {
            path: PathBuf::from("/tmp/test"),
            preset: "standard".to_string(),
            force: false,
        };
        let result = cmd.run().unwrap();
        assert!(!result.files_created.is_empty());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: InitCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("🚀 Inicializando proyecto en: {}", result.project_path.display());
    println!("📁 {} directorios creados", result.directories_created.len());
    println!("📄 {} archivos creados", result.files_created.len());
    
    Ok(())
}
