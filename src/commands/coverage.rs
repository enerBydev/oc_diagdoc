//! Comando coverage - Análisis de cobertura de contenido.
//!
//! Analiza la completitud de documentos por rangos de palabras.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// COVERAGE RANGES
// ═══════════════════════════════════════════════════════════════════════════

/// Rango de cobertura por palabras.
#[derive(Debug, Clone, Serialize)]
pub struct CoverageRange {
    pub name: String,
    pub min_words: usize,
    pub max_words: Option<usize>,
    pub count: usize,
    pub emoji: String,
}

impl CoverageRange {
    pub fn new(name: &str, min: usize, max: Option<usize>, emoji: &str) -> Self {
        Self {
            name: name.to_string(),
            min_words: min,
            max_words: max,
            count: 0,
            emoji: emoji.to_string(),
        }
    }

    pub fn contains(&self, words: usize) -> bool {
        words >= self.min_words && self.max_words.map_or(true, |max| words <= max)
    }
}

/// Resultado de cobertura.
#[derive(Debug, Clone, Serialize)]
pub struct CoverageResult {
    pub ranges: Vec<CoverageRange>,
    pub total_documents: usize,
    pub total_words: usize,
    pub avg_words: usize,
}

impl CoverageResult {
    pub fn new() -> Self {
        Self {
            ranges: vec![
                CoverageRange::new("Vacío", 0, Some(0), "⬛"),
                CoverageRange::new("Crítico", 1, Some(50), "🟥"),
                CoverageRange::new("Bajo", 51, Some(100), "🟧"),
                CoverageRange::new("Mínimo", 101, Some(200), "🟨"),
                CoverageRange::new("Aceptable", 201, Some(300), "🟩"),
                CoverageRange::new("Bueno", 301, Some(500), "🟦"),
                CoverageRange::new("Completo", 501, Some(1000), "🟪"),
                CoverageRange::new("Extenso", 1001, None, "⬜"),
            ],
            total_documents: 0,
            total_words: 0,
            avg_words: 0,
        }
    }

    pub fn add_document(&mut self, words: usize) {
        self.total_documents += 1;
        self.total_words += words;

        for range in &mut self.ranges {
            if range.contains(words) {
                range.count += 1;
                break;
            }
        }

        self.avg_words = if self.total_documents > 0 {
            self.total_words / self.total_documents
        } else {
            0
        };
    }

    pub fn coverage_percent(&self) -> f64 {
        if self.total_documents == 0 {
            return 100.0;
        }

        // Documentos con >= 200 palabras se consideran "cubiertos"
        let covered: usize = self
            .ranges
            .iter()
            .filter(|r| r.min_words >= 200)
            .map(|r| r.count)
            .sum();

        (covered as f64 / self.total_documents as f64) * 100.0
    }
}

impl Default for CoverageResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COVERAGE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de cobertura.
#[derive(Parser, Debug, Clone)]
#[command(name = "coverage", about = "Análisis de cobertura")]
pub struct CoverageCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Filtro de módulo.
    #[arg(short, long)]
    pub module: Option<String>,

    /// Output JSON.
    #[arg(long)]
    pub json: bool,
}

impl CoverageCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<CoverageResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};

        let mut result = CoverageResult::new();

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        for file_path in files {
            if let Ok(content) = read_file_content(&file_path) {
                let words = content.split_whitespace().count();
                result.add_document(words);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_result_new() {
        let result = CoverageResult::new();
        assert_eq!(result.ranges.len(), 8);
    }

    #[test]
    fn test_range_contains() {
        let range = CoverageRange::new("Test", 100, Some(200), "🟦");

        assert!(!range.contains(50));
        assert!(range.contains(100));
        assert!(range.contains(150));
        assert!(range.contains(200));
        assert!(!range.contains(201));
    }

    #[test]
    fn test_add_document() {
        let mut result = CoverageResult::new();
        result.add_document(150);
        result.add_document(350);

        assert_eq!(result.total_documents, 2);
        assert_eq!(result.avg_words, 250);
    }

    #[test]
    fn test_coverage_percent() {
        let mut result = CoverageResult::new();
        result.add_document(50); // Bajo
        result.add_document(300); // Aceptable (cubierto)

        assert_eq!(result.coverage_percent(), 50.0);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: CoverageCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📊 Cobertura de Contenido\n");

        for range in &result.ranges {
            let bar_width =
                (range.count as f64 / result.total_documents.max(1) as f64 * 20.0) as usize;
            let bar: String = "█".repeat(bar_width) + &"░".repeat(20 - bar_width);

            println!(
                "{} {:10} [{:3}] {}",
                range.emoji, range.name, range.count, bar
            );
        }

        println!("\n📈 Cobertura total: {:.1}%", result.coverage_percent());
        println!("📝 Promedio: {} palabras/doc", result.avg_words);
    }

    Ok(())
}
