//! Comando readme - Generación de README.
//!
//! Genera README automático basado en el proyecto.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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

    // L31-L32: Flags avanzados
    /// Template de README personalizado.
    #[arg(long)]
    pub template: Option<PathBuf>,

    /// Incluir estadísticas del proyecto.
    #[arg(long)]
    pub stats: bool,
}

impl ReadmeCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<ReadmeResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashMap;

        let output_path = self
            .output
            .clone()
            .unwrap_or_else(|| data_dir.join("README.md"));
        let mut result = ReadmeResult::new(output_path.clone());

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let module_regex = Regex::new(r#"module:\s*["']?([^"'\n]+)["']?"#).unwrap();

        // Recolectar stats
        let mut module_stats: HashMap<String, usize> = HashMap::new();
        let mut total_words = 0usize;

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let module = module_regex
                    .captures(&content)
                    .map(|c| c[1].trim().to_string())
                    .unwrap_or_else(|| "sin_modulo".to_string());

                *module_stats.entry(module).or_insert(0) += 1;
                total_words += content.split_whitespace().count();
            }
        }

        // L32.1: Cargar template si existe
        let template_content = if let Some(ref template_path) = self.template {
            std::fs::read_to_string(template_path).ok()
        } else {
            None
        };

        // Generar README
        let mut readme = String::new();

        // Badges
        if self.badges {
            readme.push_str("![Docs](https://img.shields.io/badge/docs-");
            readme.push_str(&files.len().to_string());
            readme.push_str("-blue)\n");
            readme.push_str("![Words](https://img.shields.io/badge/words-");
            readme.push_str(&total_words.to_string());
            readme.push_str("-green)\n\n");
            result.sections_generated += 1;
        }

        // Header
        readme.push_str("# Documentación del Proyecto\n\n");
        result.sections_generated += 1;

        // L31.1: Stats reales
        if self.stats {
            readme.push_str("## 📊 Estadísticas\n\n");
            readme.push_str(&format!("- **Documentos:** {}\n", files.len()));
            readme.push_str(&format!("- **Palabras:** {}\n", total_words));
            readme.push_str(&format!("- **Módulos:** {}\n\n", module_stats.len()));
            result.sections_generated += 1;
        }

        // L31.2: TOC de módulos
        if self.toc {
            readme.push_str("## 📁 Módulos\n\n");
            readme.push_str("| Módulo | Documentos |\n");
            readme.push_str("|--------|------------|\n");

            let mut modules: Vec<_> = module_stats.iter().collect();
            modules.sort_by(|a, b| b.1.cmp(a.1));

            for (module, count) in modules {
                readme.push_str(&format!("| {} | {} |\n", module, count));
            }
            readme.push('\n');
            result.sections_generated += 1;
        }

        // L32.1: Aplicar template si existe
        if let Some(template) = template_content {
            let content = template
                .replace("{{PROJECT_NAME}}", "Proyecto")
                .replace("{{DOC_COUNT}}", &files.len().to_string())
                .replace("{{WORD_COUNT}}", &total_words.to_string())
                .replace("{{MODULE_COUNT}}", &module_stats.len().to_string())
                .replace(
                    "{{DATE}}",
                    &chrono::Utc::now().format("%Y-%m-%d").to_string(),
                );
            readme.push_str(&content);
            result.sections_generated += 1;
        }

        // Footer
        readme.push_str("---\n\n");
        readme.push_str(&format!(
            "*Generado automáticamente por oc_diagdoc el {}*\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M")
        ));
        result.sections_generated += 1;

        result.lines = readme.lines().count();

        // Escribir archivo
        std::fs::write(&output_path, &readme)?;

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
    fn test_readme_command_fields() {
        let cmd = ReadmeCommand {
            path: None,
            output: Some(PathBuf::from("README.md")),
            badges: true,
            toc: true,
            template: None,
            stats: true,
        };
        assert!(cmd.stats);
        assert!(cmd.badges);
    }

    #[test]
    fn test_readme_with_options() {
        let cmd = ReadmeCommand {
            path: None,
            output: None,
            badges: true,
            toc: true,
            template: Some(PathBuf::from("template.md")),
            stats: false,
        };
        assert!(cmd.template.is_some());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ReadmeCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    println!("📝 README generado: {}", result.output_path.display());
    println!(
        "📊 {} secciones, {} líneas",
        result.sections_generated, result.lines
    );

    Ok(())
}
