//! Comando export - Exportación de documentación.
//!
//! Exporta documentación a múltiples formatos.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Formato de exportación.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Markdown,
    Html,
    Pdf,
    Docx,
    Json,
    Latex,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "md" | "markdown" => Some(Self::Markdown),
            "html" => Some(Self::Html),
            "pdf" => Some(Self::Pdf),
            "docx" | "word" => Some(Self::Docx),
            "json" => Some(Self::Json),
            "latex" | "tex" => Some(Self::Latex),
            _ => None,
        }
    }
    
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Html => "html",
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Json => "json",
            Self::Latex => "tex",
        }
    }
}

/// Resultado de exportación.
#[derive(Debug, Clone, Serialize)]
pub struct ExportResult {
    pub output_path: PathBuf,
    pub format: String,
    pub files_exported: usize,
    pub total_bytes: usize,
}

impl ExportResult {
    pub fn new(path: PathBuf, format: &str) -> Self {
        Self {
            output_path: path,
            format: format.to_string(),
            files_exported: 0,
            total_bytes: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de exportación.
#[derive(Parser, Debug, Clone)]
#[command(name = "export", about = "Exportar documentación")]
pub struct ExportCommand {
    /// Formato de salida.
    #[arg(short, long, default_value = "markdown")]
    pub format: String,
    
    /// Ruta de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Incluir tabla de contenidos.
    #[arg(long)]
    pub toc: bool,
}

impl ExportCommand {
    pub fn run(&self) -> OcResult<ExportResult> {
        let output = self.output.clone().unwrap_or_else(|| {
            PathBuf::from(format!("export.{}", self.format_enum().extension()))
        });
        let result = ExportResult::new(output, &self.format);
        // TODO: Implementar exportación real
        Ok(result)
    }
    
    pub fn format_enum(&self) -> ExportFormat {
        ExportFormat::from_str(&self.format).unwrap_or(ExportFormat::Markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_str() {
        assert_eq!(ExportFormat::from_str("pdf"), Some(ExportFormat::Pdf));
        assert_eq!(ExportFormat::from_str("html"), Some(ExportFormat::Html));
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Pdf.extension(), "pdf");
        assert_eq!(ExportFormat::Latex.extension(), "tex");
    }

    #[test]
    fn test_export_result_new() {
        let result = ExportResult::new(PathBuf::from("out.pdf"), "pdf");
        assert_eq!(result.format, "pdf");
    }

    #[test]
    fn test_export_command_format_enum() {
        let cmd = ExportCommand {
            format: "latex".to_string(),
            output: None,
            path: None,
            toc: false,
        };
        assert_eq!(cmd.format_enum(), ExportFormat::Latex);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ExportCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("📤 Exportando a formato: {}", result.format);
    println!("📁 Salida: {}", result.output_path.display());
    println!("📊 {} archivos, {} bytes", result.files_exported, result.total_bytes);
    
    Ok(())
}
