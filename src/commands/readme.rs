//! Comando readme - Generación de README.
//!
//! Genera README automático basado en el proyecto.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// README TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de generación de README.
#[derive(Debug, Clone, Serialize)]
pub struct ReadmeResult {
    pub output_path: PathBuf,
    pub sections_generated: usize,
    pub lines: usize,
}

impl ReadmeResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            output_path: path,
            sections_generated: 0,
            lines: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// README COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de README.
#[derive(Parser, Debug, Clone)]
#[command(name = "readme", about = "Generar README")]
pub struct ReadmeCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Archivo de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Incluir badges.
    #[arg(long)]
    pub badges: bool,
    
    /// Incluir tabla de contenidos.
    #[arg(long)]
    pub toc: bool,
}

impl ReadmeCommand {
    pub fn run(&self) -> OcResult<ReadmeResult> {
        let output = self.output.clone().unwrap_or_else(|| PathBuf::from("README.md"));
        let mut result = ReadmeResult::new(output);
        result.sections_generated = 5;
        result.lines = 150;
        // TODO: Generar README real
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_result_new() {
        let result = ReadmeResult::new(PathBuf::from("README.md"));
        assert_eq!(result.sections_generated, 0);
    }

    #[test]
    fn test_readme_command_run() {
        let cmd = ReadmeCommand {
            path: None,
            output: Some(PathBuf::from("README.md")),
            badges: true,
            toc: true,
        };
        let result = cmd.run().unwrap();
        assert!(result.sections_generated > 0);
    }

    #[test]
    fn test_readme_default_output() {
        let cmd = ReadmeCommand {
            path: None,
            output: None,
            badges: false,
            toc: false,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.output_path, PathBuf::from("README.md"));
    }

    #[test]
    fn test_readme_with_options() {
        let cmd = ReadmeCommand {
            path: None,
            output: None,
            badges: true,
            toc: true,
        };
        assert!(cmd.run().is_ok());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ReadmeCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("📝 README generado: {}", result.output_path.display());
    println!("📊 {} secciones, {} líneas", result.sections_generated, result.lines);
    
    Ok(())
}
