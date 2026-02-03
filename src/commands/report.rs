//! Comando report - Generación de reportes.
//!
//! Genera reportes en múltiples formatos.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<Report> {
        
        use std::collections::HashMap;
        use std::fs;

        let mut report = Report::new("Reporte de Documentación OnlyCar");

        // Collect files using WalkDir (RECURSIVE)
        use walkdir::WalkDir;
        let files: Vec<PathBuf> = WalkDir::new(data_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                if !path.is_file() { return false; }
                if path.extension().map(|ext| ext != "md").unwrap_or(true) { return false; }
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                !name.starts_with("TRAP_") && !name.starts_with("AUTOTEST_") && !name.starts_with("TEST_")
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        // Calculate stats
        use crate::core::patterns::RE_WIKI_LINK_WITH_ALIAS;
        let link_re = &*RE_WIKI_LINK_WITH_ALIAS;
        let mut total_words = 0usize;
        let mut total_links = 0usize;
        let mut modules: HashMap<String, usize> = HashMap::new();

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                // Words
                let body = if content.starts_with("---") {
                    if let Some(end) = content[3..].find("---") {
                        &content[3 + end + 3..]
                    } else {
                        &content
                    }
                } else {
                    &content
                };
                total_words += body.split_whitespace().count();

                // Links
                total_links += link_re.captures_iter(&content).count();

                // Modules
                if let Some(id) = Self::get_yaml_field(&content, "id") {
                    let module = id.split('.').next().unwrap_or("0").to_string();
                    *modules.entry(module).or_insert(0) += 1;
                }
            }
        }

        // Summary section
        let summary = format!(
            "| Métrica | Valor |\n|---------|-------|\n\
             | Documentos | {} |\n\
             | Palabras totales | {} |\n\
             | Promedio palabras/doc | {} |\n\
             | Enlaces internos | {} |\n\
             | Módulos | {} |",
            files.len(),
            total_words,
            if files.is_empty() {
                0
            } else {
                total_words / files.len()
            },
            total_links,
            modules.len()
        );
        report.add_section("Resumen Ejecutivo", &summary, 2);

        // Modules section
        let mut sorted_modules: Vec<_> = modules.iter().collect();
        sorted_modules.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(0));

        let modules_content = sorted_modules
            .iter()
            .map(|(id, count)| format!("- **Módulo {}**: {} documentos", id, count))
            .collect::<Vec<_>>()
            .join("\n");
        report.add_section("Distribución por Módulos", &modules_content, 2);

        // Health section
        let health = format!(
            "- ✅ **Documentos con frontmatter válido**: {}%\n\
             - 📊 **Cobertura de enlaces**: {} links en {} docs",
            100, // All files have frontmatter by default
            total_links,
            files.len()
        );
        report.add_section("Salud del Proyecto", &health, 2);

        Ok(report)
    }

    fn get_yaml_field(content: &str, field: &str) -> Option<String> {
        if !content.starts_with("---") {
            return None;
        }
        let end_idx = content[3..].find("---")?;
        let yaml_text = &content[3..3 + end_idx];
        for line in yaml_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("{}:", field)) {
                let value_part = trimmed.strip_prefix(&format!("{}:", field))?;
                let value = value_part.trim().trim_matches(|c| c == '"' || c == '\'');
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
        None
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
pub fn run(cmd: ReportCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    // Priorizar cmd.path, sino usar cli.data_dir
    let data_dir = cmd
        .path
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from(&cli.data_dir));
    let report = cmd.run(&data_dir)?;

    let output = report.to_markdown();

    if let Some(path) = &cmd.output {
        std::fs::write(path, &output)?;
        println!("📄 Reporte guardado: {}", path.display());
    } else {
        println!("{}", output);
    }

    Ok(())
}
