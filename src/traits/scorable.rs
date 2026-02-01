//! Trait Scorable - Para cálculo de puntuaciones de documentos.

/// Trait para elementos que pueden ser puntuados.
pub trait Scorable {
    /// Puntuación de salud (0.0 - 100.0).
    fn health_score(&self) -> f64;
    
    /// Puntuación de completitud (0.0 - 100.0).
    fn completeness_score(&self) -> f64;
    
    /// Puntuación de calidad (0.0 - 100.0).
    fn quality_score(&self) -> f64 {
        (self.health_score() + self.completeness_score()) / 2.0
    }
    
    /// ¿Está saludable? (score > 80).
    fn is_healthy(&self) -> bool {
        self.health_score() > 80.0
    }
    
    /// ¿Está completo? (score > 90).
    fn is_complete(&self) -> bool {
        self.completeness_score() > 90.0
    }
    
    /// Nivel de alerta basado en score.
    fn alert_level(&self) -> AlertLevel {
        let score = self.quality_score();
        if score >= 80.0 {
            AlertLevel::None
        } else if score >= 60.0 {
            AlertLevel::Warning
        } else if score >= 40.0 {
            AlertLevel::High
        } else {
            AlertLevel::Critical
        }
    }
}

/// Nivel de alerta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertLevel {
    None,
    Warning,
    High,
    Critical,
}

impl AlertLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            AlertLevel::None => "✅",
            AlertLevel::Warning => "⚠️",
            AlertLevel::High => "🔶",
            AlertLevel::Critical => "🔴",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockDocument {
        health: f64,
        completeness: f64,
    }
    
    impl Scorable for MockDocument {
        fn health_score(&self) -> f64 { self.health }
        fn completeness_score(&self) -> f64 { self.completeness }
    }
    
    #[test]
    fn test_quality_score() {
        let doc = MockDocument { health: 80.0, completeness: 100.0 };
        assert_eq!(doc.quality_score(), 90.0);
    }
    
    #[test]
    fn test_alert_level() {
        let healthy = MockDocument { health: 90.0, completeness: 90.0 };
        assert_eq!(healthy.alert_level(), AlertLevel::None);
        
        let critical = MockDocument { health: 20.0, completeness: 20.0 };
        assert_eq!(critical.alert_level(), AlertLevel::Critical);
    }
}
