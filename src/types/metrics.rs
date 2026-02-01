//! Estructuras de métricas y estadísticas.

use serde::{Deserialize, Serialize};

/// Contadores generales de la colección.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Counters {
    pub total_files: usize,
    pub total_words: usize,
    pub total_lines: usize,
    pub total_links: usize,
    pub total_modules: usize,
    pub orphan_files: usize,
    pub broken_links: usize,
}

impl Counters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Promedio de palabras por archivo.
    pub fn avg_words_per_file(&self) -> f64 {
        if self.total_files > 0 {
            self.total_words as f64 / self.total_files as f64
        } else {
            0.0
        }
    }
}

/// Estadísticas de cobertura por rango de palabras.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoverageStats {
    /// 🔴 < 50 palabras
    pub critical: usize,
    /// 🟠 50-99 palabras
    pub alert: usize,
    /// 🟡 100-149 palabras
    pub warning: usize,
    /// 🟤 150-199 palabras
    pub low: usize,
    /// 🔵 200-249 palabras
    pub moderate: usize,
    /// 🟣 250-299 palabras
    pub almost: usize,
    /// ⚪ 300-349 palabras
    pub pending: usize,
    /// 🟢 350+ palabras
    pub acceptable: usize,
}

impl CoverageStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clasifica un conteo de palabras.
    pub fn classify(&mut self, word_count: usize) {
        match word_count {
            0..=49 => self.critical += 1,
            50..=99 => self.alert += 1,
            100..=149 => self.warning += 1,
            150..=199 => self.low += 1,
            200..=249 => self.moderate += 1,
            250..=299 => self.almost += 1,
            300..=349 => self.pending += 1,
            _ => self.acceptable += 1,
        }
    }

    /// Total de archivos.
    pub fn total(&self) -> usize {
        self.critical
            + self.alert
            + self.warning
            + self.low
            + self.moderate
            + self.almost
            + self.pending
            + self.acceptable
    }

    /// Porcentaje de aceptables.
    pub fn percent_acceptable(&self) -> f64 {
        let total = self.total();
        if total > 0 {
            (self.acceptable as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Porcentaje de críticos.
    pub fn percent_critical(&self) -> f64 {
        let total = self.total();
        if total > 0 {
            (self.critical as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Score de salud (0-100).
    pub fn health_score(&self) -> u8 {
        let total = self.total() as f64;
        if total == 0.0 {
            return 0;
        }

        // Ponderación: aceptables valen más, críticos restan
        let score = (self.acceptable as f64 * 1.0
            + self.pending as f64 * 0.85
            + self.almost as f64 * 0.7
            + self.moderate as f64 * 0.55
            + self.low as f64 * 0.4
            + self.warning as f64 * 0.25
            + self.alert as f64 * 0.1
            + self.critical as f64 * 0.0)
            / total
            * 100.0;

        score.round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage_classify() {
        let mut stats = CoverageStats::new();
        stats.classify(30); // critical
        stats.classify(75); // alert
        stats.classify(400); // acceptable

        assert_eq!(stats.critical, 1);
        assert_eq!(stats.alert, 1);
        assert_eq!(stats.acceptable, 1);
        assert_eq!(stats.total(), 3);
    }

    #[test]
    fn test_health_score() {
        let mut stats = CoverageStats::new();
        for _ in 0..10 {
            stats.classify(400); // all acceptable
        }
        assert_eq!(stats.health_score(), 100);
    }
}
