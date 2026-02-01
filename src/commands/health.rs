//! Comando health - Salud general del proyecto.
//!
//! Dashboard de salud con múltiples métricas.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH METRICS
// ═══════════════════════════════════════════════════════════════════════════

/// Métrica individual de salud.
#[derive(Debug, Clone, Serialize)]
pub struct HealthMetric {
    pub name: String,
    pub value: u8, // 0-100
    pub weight: f32,
    pub status: String,
}

impl HealthMetric {
    pub fn new(name: &str, value: u8, weight: f32) -> Self {
        let status = match value {
            90..=100 => "🟢 Excelente",
            70..=89 => "🟡 Bien",
            50..=69 => "🟠 Regular",
            _ => "🔴 Crítico",
        };
        Self {
            name: name.to_string(),
            value,
            weight,
            status: status.to_string(),
        }
    }
}

/// Resultado de salud.
#[derive(Debug, Clone, Serialize)]
pub struct HealthResult {
    pub metrics: Vec<HealthMetric>,
    pub overall_score: u8,
    pub grade: char,
}

impl HealthResult {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            overall_score: 100,
            grade: 'A',
        }
    }

    pub fn add_metric(&mut self, metric: HealthMetric) {
        self.metrics.push(metric);
        self.calculate_overall();
    }

    fn calculate_overall(&mut self) {
        if self.metrics.is_empty() {
            return;
        }

        let total_weight: f32 = self.metrics.iter().map(|m| m.weight).sum();
        let weighted_sum: f32 = self.metrics.iter().map(|m| m.value as f32 * m.weight).sum();

        self.overall_score = if total_weight > 0.0 {
            (weighted_sum / total_weight).round() as u8
        } else {
            100
        };

        self.grade = match self.overall_score {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        };
    }

    pub fn is_healthy(&self) -> bool {
        self.overall_score >= 70
    }
}

impl Default for HealthResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de salud.
#[derive(Parser, Debug, Clone)]
#[command(name = "health", about = "Dashboard de salud")]
pub struct HealthCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Output JSON.
    #[arg(long)]
    pub json: bool,

    /// Detallado.
    #[arg(short, long)]
    pub verbose: bool,
}

impl HealthCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<HealthResult> {
        use crate::core::loader::quick_stats;

        let mut result = HealthResult::new();

        // Calcular métricas basadas en estadísticas reales
        let qs = quick_stats(data_dir)?;

        // Cobertura: % de docs con frontmatter
        let coverage = if qs.file_count > 0 {
            (qs.with_frontmatter as f64 / qs.file_count as f64 * 100.0) as u8
        } else {
            100
        };

        // Estructura: basado en promedio de palabras (>300 = bueno)
        let avg_words = qs.avg_words_per_file();
        let structure = if avg_words >= 500 {
            100
        } else if avg_words >= 300 {
            85
        } else if avg_words >= 100 {
            60
        } else {
            30
        };

        result.add_metric(HealthMetric::new("Cobertura", coverage, 1.0));
        result.add_metric(HealthMetric::new("Enlaces", 95, 0.8)); // Pendiente análisis real
        result.add_metric(HealthMetric::new("Estructura", structure, 1.2));
        result.add_metric(HealthMetric::new("Metadatos", coverage, 0.5));

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_result_new() {
        let result = HealthResult::new();
        assert!(result.is_healthy());
    }

    #[test]
    fn test_health_metric_new() {
        let metric = HealthMetric::new("Test", 85, 1.0);
        assert!(metric.status.contains("Bien"));
    }

    #[test]
    fn test_calculate_overall() {
        let mut result = HealthResult::new();
        result.add_metric(HealthMetric::new("A", 100, 1.0));
        result.add_metric(HealthMetric::new("B", 0, 1.0));

        assert_eq!(result.overall_score, 50);
        assert_eq!(result.grade, 'F');
    }

    #[test]
    fn test_grade_calculation() {
        let mut result = HealthResult::new();
        result.add_metric(HealthMetric::new("A", 95, 1.0));

        assert_eq!(result.grade, 'A');
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: HealthCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("╔══════════════════════════════════════╗");
        println!("║          🏥 HEALTH DASHBOARD         ║");
        println!("╠══════════════════════════════════════╣");

        for m in &result.metrics {
            println!("║ {:15} {:3}% {}      ║", m.name, m.value, &m.status[..6]);
        }

        println!("╠══════════════════════════════════════╣");
        println!(
            "║ OVERALL:         {:3}% Grade: {}       ║",
            result.overall_score, result.grade
        );
        println!("╚══════════════════════════════════════╝");
    }

    Ok(())
}
