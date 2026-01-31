//! Comando stats - Dashboard de estadísticas.
//!
//! Muestra estadísticas completas del proyecto.

use std::path::PathBuf;
use std::collections::HashMap;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// STATS OUTPUT
// ═══════════════════════════════════════════════════════════════════════════

/// Estadísticas globales del proyecto.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectStats {
    /// Total de documentos.
    pub total_documents: usize,
    /// Documentos saludables.
    pub healthy_documents: usize,
    /// Total de palabras.
    pub total_words: usize,
    /// Total de enlaces.
    pub total_links: usize,
    /// Enlaces rotos.
    pub broken_links: usize,
    /// Módulos.
    pub modules_count: usize,
    /// Profundidad máxima.
    pub max_depth: usize,
}

impl ProjectStats {
    pub fn new() -> Self {
        Self {
            total_documents: 0,
            healthy_documents: 0,
            total_words: 0,
            total_links: 0,
            broken_links: 0,
            modules_count: 0,
            max_depth: 0,
        }
    }
    
    pub fn health_percent(&self) -> f64 {
        if self.total_documents == 0 {
            100.0
        } else {
            (self.healthy_documents as f64 / self.total_documents as f64) * 100.0
        }
    }
    
    pub fn avg_words_per_doc(&self) -> usize {
        if self.total_documents == 0 {
            0
        } else {
            self.total_words / self.total_documents
        }
    }
}

impl Default for ProjectStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Estadísticas por módulo.
#[derive(Debug, Clone, Serialize)]
pub struct ModuleStats {
    pub id: String,
    pub name: String,
    pub document_count: usize,
    pub word_count: usize,
    pub health_score: f64,
}

// ═══════════════════════════════════════════════════════════════════════════
// STATS COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de estadísticas.
#[derive(Parser, Debug, Clone)]
#[command(name = "stats", about = "Dashboard de estadísticas del proyecto")]
pub struct StatsCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Mostrar por módulo.
    #[arg(short, long)]
    pub by_module: bool,
    
    /// Output JSON.
    #[arg(long)]
    pub json: bool,
    
    /// Ordenar por campo.
    #[arg(long, default_value = "id")]
    pub sort: String,
}

impl StatsCommand {
    /// Ejecuta el comando.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<(ProjectStats, Vec<ModuleStats>)> {
        use crate::core::loader::quick_stats;
        
        // Cargar estadísticas rápidas
        let qs = quick_stats(data_dir)?;
        
        let project_stats = ProjectStats {
            total_documents: qs.file_count,
            healthy_documents: qs.with_frontmatter,
            total_words: qs.total_words,
            total_links: 0, // TODO: contar links
            broken_links: 0,
            modules_count: 0, // TODO: contar módulos
            max_depth: 0,
        };
        
        let module_stats = Vec::new();
        
        Ok((project_stats, module_stats))
    }
    
    /// Renderiza como tabla.
    pub fn render_table(stats: &ProjectStats) -> String {
        format!(
            "╔══════════════════════════════════════╗\n\
             ║           📊 PROJECT STATS           ║\n\
             ╠══════════════════════════════════════╣\n\
             ║ Documents:     {:>20} ║\n\
             ║ Healthy:       {:>20} ║\n\
             ║ Health:        {:>19.1}% ║\n\
             ║ Words:         {:>20} ║\n\
             ║ Avg Words/Doc: {:>20} ║\n\
             ║ Links:         {:>20} ║\n\
             ║ Broken Links:  {:>20} ║\n\
             ║ Modules:       {:>20} ║\n\
             ║ Max Depth:     {:>20} ║\n\
             ╚══════════════════════════════════════╝",
            stats.total_documents,
            stats.healthy_documents,
            stats.health_percent(),
            stats.total_words,
            stats.avg_words_per_doc(),
            stats.total_links,
            stats.broken_links,
            stats.modules_count,
            stats.max_depth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_stats_new() {
        let stats = ProjectStats::new();
        assert_eq!(stats.total_documents, 0);
    }

    #[test]
    fn test_health_percent() {
        let mut stats = ProjectStats::new();
        stats.total_documents = 100;
        stats.healthy_documents = 75;
        
        assert_eq!(stats.health_percent(), 75.0);
    }

    #[test]
    fn test_avg_words() {
        let mut stats = ProjectStats::new();
        stats.total_documents = 10;
        stats.total_words = 1000;
        
        assert_eq!(stats.avg_words_per_doc(), 100);
    }

    #[test]
    fn test_render_table() {
        let stats = ProjectStats::new();
        let output = StatsCommand::render_table(&stats);
        
        assert!(output.contains("PROJECT STATS"));
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: StatsCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let (stats, module_stats) = cmd.run(data_dir)?;
    
    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("{}", StatsCommand::render_table(&stats));
        
        if cmd.by_module && !module_stats.is_empty() {
            println!("\n📦 Stats por módulo:");
            for ms in &module_stats {
                println!("  {} ({}): {} docs, {} words", ms.name, ms.id, ms.document_count, ms.word_count);
            }
        }
    }
    
    Ok(())
}
