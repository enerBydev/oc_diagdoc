//! Comando archive - Archivado de documentos.
//!
//! Archiva documentos obsoletos o completados.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// ARCHIVE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de archivado.
#[derive(Debug, Clone, Serialize)]
pub struct ArchiveResult {
    pub archive_path: PathBuf,
    pub files_archived: usize,
    pub total_bytes: usize,
}

impl ArchiveResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            archive_path: path,
            files_archived: 0,
            total_bytes: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ARCHIVE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de archivado.
#[derive(Parser, Debug, Clone)]
#[command(name = "archive", about = "Archivar documentos")]
pub struct ArchiveCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Archivo de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Archivar documentos con status específico.
    #[arg(long)]
    pub status: Option<String>,
    
    /// Comprimir archivo.
    #[arg(long)]
    pub compress: bool,
}

impl ArchiveCommand {
    pub fn run(&self) -> OcResult<ArchiveResult> {
        let output = self.output.clone().unwrap_or_else(|| {
            PathBuf::from("archive.tar.gz")
        });
        let result = ArchiveResult::new(output);
        // TODO: Implementar archivado real
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_result_new() {
        let result = ArchiveResult::new(PathBuf::from("out.tar.gz"));
        assert_eq!(result.files_archived, 0);
    }

    #[test]
    fn test_archive_command_run() {
        let cmd = ArchiveCommand {
            path: None,
            output: Some(PathBuf::from("test.tar.gz")),
            status: None,
            compress: true,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.archive_path, PathBuf::from("test.tar.gz"));
    }

    #[test]
    fn test_archive_default_output() {
        let cmd = ArchiveCommand {
            path: None,
            output: None,
            status: None,
            compress: false,
        };
        let result = cmd.run().unwrap();
        assert!(result.archive_path.to_str().unwrap().ends_with(".tar.gz"));
    }

    #[test]
    fn test_archive_with_status() {
        let cmd = ArchiveCommand {
            path: None,
            output: None,
            status: Some("deprecated".to_string()),
            compress: true,
        };
        assert!(cmd.run().is_ok());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ArchiveCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("📦 Archivando en: {}", result.archive_path.display());
    println!("📄 {} archivos, {} bytes", result.files_archived, result.total_bytes);
    
    Ok(())
}
