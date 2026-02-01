//! Memoria de ejecuciones para aprendizaje.
//!
//! Almacena historial y métricas para optimización.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════
// EXECUTION RECORD
// ═══════════════════════════════════════════════════════════════════════════

/// Registro de una ejecución.
#[derive(Debug, Clone)]
pub struct ExecutionRecord {
    /// Nombre del comando.
    pub command: String,
    /// Timestamp.
    pub timestamp: Instant,
    /// Duración.
    pub duration: Duration,
    /// Documentos procesados.
    pub docs_processed: usize,
    /// Éxito.
    pub success: bool,
}

impl ExecutionRecord {
    pub fn new(
        command: impl Into<String>,
        duration: Duration,
        docs_processed: usize,
        success: bool,
    ) -> Self {
        Self {
            command: command.into(),
            timestamp: Instant::now(),
            duration,
            docs_processed,
            success,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EXECUTION STATS
// ═══════════════════════════════════════════════════════════════════════════

/// Estadísticas acumuladas.
#[derive(Debug, Clone, Default)]
pub struct ExecutionStats {
    /// Total de ejecuciones.
    pub total_runs: usize,
    /// Ejecuciones exitosas.
    pub successful_runs: usize,
    /// Tiempo total.
    pub total_time: Duration,
    /// Documentos totales.
    pub total_docs: usize,
}

impl ExecutionStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successful_runs as f64 / self.total_runs as f64 * 100.0
        }
    }

    pub fn avg_duration(&self) -> Duration {
        if self.total_runs == 0 {
            Duration::ZERO
        } else {
            self.total_time / self.total_runs as u32
        }
    }

    pub fn docs_per_second(&self) -> f64 {
        let secs = self.total_time.as_secs_f64();
        if secs == 0.0 {
            0.0
        } else {
            self.total_docs as f64 / secs
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EXECUTION MEMORY
// ═══════════════════════════════════════════════════════════════════════════

/// Memoria de ejecuciones.
#[derive(Debug)]
pub struct ExecutionMemory {
    /// Historial reciente.
    history: VecDeque<ExecutionRecord>,
    /// Tamaño máximo del historial.
    max_history: usize,
    /// Estadísticas acumuladas.
    stats: ExecutionStats,
}

impl ExecutionMemory {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 100,
            stats: ExecutionStats::default(),
        }
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// Registra una ejecución.
    pub fn record(&mut self, record: ExecutionRecord) {
        // Actualizar stats
        self.stats.total_runs += 1;
        if record.success {
            self.stats.successful_runs += 1;
        }
        self.stats.total_time += record.duration;
        self.stats.total_docs += record.docs_processed;

        // Agregar al historial
        self.history.push_back(record);

        // Truncar si excede
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Estadísticas actuales.
    pub fn stats(&self) -> &ExecutionStats {
        &self.stats
    }

    /// Historial reciente.
    pub fn recent(&self, n: usize) -> Vec<&ExecutionRecord> {
        self.history.iter().rev().take(n).collect()
    }

    /// Tendencia de duración (últimas N vs promedio).
    pub fn duration_trend(&self, last_n: usize) -> f64 {
        if self.history.len() < last_n {
            return 0.0;
        }

        let recent_avg: Duration = self
            .history
            .iter()
            .rev()
            .take(last_n)
            .map(|r| r.duration)
            .sum::<Duration>()
            / last_n as u32;

        let overall_avg = self.stats.avg_duration();

        if overall_avg.as_secs_f64() == 0.0 {
            0.0
        } else {
            (recent_avg.as_secs_f64() - overall_avg.as_secs_f64()) / overall_avg.as_secs_f64()
                * 100.0
        }
    }

    /// Sugiere optimizaciones basadas en historial.
    pub fn suggest_optimizations(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if self.stats.success_rate() < 90.0 {
            suggestions.push("Consider running validation before commands".to_string());
        }

        if self.stats.docs_per_second() < 10.0 && self.stats.total_runs > 5 {
            suggestions.push("Enable caching for better performance".to_string());
        }

        suggestions
    }
}

impl Default for ExecutionMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_memory_record() {
        let mut memory = ExecutionMemory::new();
        memory.record(ExecutionRecord::new(
            "test",
            Duration::from_secs(1),
            10,
            true,
        ));

        assert_eq!(memory.stats().total_runs, 1);
        assert_eq!(memory.stats().successful_runs, 1);
    }

    #[test]
    fn test_execution_stats() {
        let mut memory = ExecutionMemory::new();
        memory.record(ExecutionRecord::new(
            "cmd1",
            Duration::from_secs(1),
            10,
            true,
        ));
        memory.record(ExecutionRecord::new(
            "cmd2",
            Duration::from_secs(1),
            10,
            false,
        ));

        assert_eq!(memory.stats().success_rate(), 50.0);
    }

    #[test]
    fn test_recent() {
        let mut memory = ExecutionMemory::new();
        for i in 0..5 {
            memory.record(ExecutionRecord::new(
                format!("cmd{}", i),
                Duration::from_secs(1),
                1,
                true,
            ));
        }

        let recent = memory.recent(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_max_history() {
        let mut memory = ExecutionMemory::new().with_max_history(5);
        for i in 0..10 {
            memory.record(ExecutionRecord::new(
                format!("cmd{}", i),
                Duration::from_secs(1),
                1,
                true,
            ));
        }

        assert_eq!(memory.history.len(), 5);
    }
}
