//! Comando report - Generación de reportes.
//!
//! Genera reportes en múltiples formatos.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// REPORT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Formato de reporte.
#[derive(Debug, Clone, PartialEq)]
pub enum ReportFormat {
    Markdown,
    Html,
    Json,
    Pdf,
}

/// Sección del reporte.
#[derive(Debug, Clone, Serialize)]
pub struct ReportSection {
    pub title: String,
    pub content: String,
    pub level: u8,
}

/// Reporte generado.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub title: String,
    pub sections: Vec<ReportSection>,
    pub generated_at: String,
}

impl Report {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            sections: Vec::new(),
            generated_at: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
        }
    }
    
    pub fn add_section(&mut self, title: &str, content: &str, level: u8) {
        self.sections.push(ReportSection {
            title: title.to_string(),
            content: content.to_string(),
            level,
        });
    }
    
    pub fn to_markdown(&self) -> String {
        let mut output = format!("# {}\n\n", self.title);
        output.push_str(&format!("*Generado: {}*\n\n", self.generated_at));
        
        for section in &self.sections {
            let heading = "#".repeat(section.level as usize);
            output.push_str(&format!("{} {}\n\n", heading, section.title));
            output.push_str(&section.content);
            output.push_str("\n\n");
        }
        
        output
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REPORT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de reporte.
#[derive(Parser, Debug, Clone)]
#[command(name = "report", about = "Generación de reportes")]
pub struct ReportCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Formato de salida.
    #[arg(short, long, default_value = "markdown")]
    pub format: String,
    
    /// Archivo de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Tipo de reporte.
    #[arg(short, long, default_value = "full")]
    pub report_type: String,
}

impl ReportCommand {
    pub fn run(&self) -> OcResult<Report> {
        let mut report = Report::new("Reporte de Documentación");
        
        // TODO: Generar contenido real
        report.add_section("Resumen", "Resumen ejecutivo del proyecto.", 2);
        report.add_section("Estadísticas", "Métricas clave del proyecto.", 2);
        
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_new() {
        let report = Report::new("Test Report");
        assert_eq!(report.title, "Test Report");
        assert!(report.sections.is_empty());
    }

    #[test]
    fn test_add_section() {
        let mut report = Report::new("Test");
        report.add_section("Section 1", "Content", 2);
        
        assert_eq!(report.sections.len(), 1);
    }

    #[test]
    fn test_to_markdown() {
        let mut report = Report::new("Test");
        report.add_section("Section", "Content here", 2);
        
        let md = report.to_markdown();
        assert!(md.contains("# Test"));
        assert!(md.contains("## Section"));
    }

    #[test]
    fn test_section_levels() {
        let mut report = Report::new("Test");
        report.add_section("H2", "c", 2);
        report.add_section("H3", "c", 3);
        
        let md = report.to_markdown();
        assert!(md.contains("## H2"));
        assert!(md.contains("### H3"));
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ReportCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let report = cmd.run()?;
    
    let output = report.to_markdown();
    
    if let Some(path) = &cmd.output {
        std::fs::write(path, &output)?;
        println!("📄 Reporte guardado: {}", path.display());
    } else {
        println!("{}", output);
    }
    
    Ok(())
}
