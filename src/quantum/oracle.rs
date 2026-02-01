//! Oráculo cuántico para predicción de errores.
//!
//! Analiza patrones históricos y predice problemas futuros.

use crate::data::project::ProjectState;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// PREDICTION TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de predicción.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredictionType {
    /// Documento que pronto estará desactualizado.
    StaleDocument,
    /// Enlace que probablemente se romperá.
    BrokenLinkRisk,
    /// Módulo con cobertura decreciente.
    CoverageDecline,
    /// Documento huérfano potencial.
    OrphanRisk,
    /// Inconsistencia de breadcrumb.
    BreadcrumbDrift,
}

/// Nivel de confianza.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Confidence {
    pub fn as_percent(&self) -> u8 {
        match self {
            Self::Low => 25,
            Self::Medium => 50,
            Self::High => 75,
            Self::VeryHigh => 95,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PREDICTION
// ═══════════════════════════════════════════════════════════════════════════

/// Una predicción del oráculo.
#[derive(Debug, Clone)]
pub struct Prediction {
    /// Tipo de predicción.
    pub prediction_type: PredictionType,
    /// Entidad afectada (ID o path).
    pub target: String,
    /// Mensaje descriptivo.
    pub message: String,
    /// Nivel de confianza.
    pub confidence: Confidence,
    /// Tiempo estimado hasta el problema (en días).
    pub days_until: Option<u32>,
    /// Acción preventiva sugerida.
    pub preventive_action: Option<String>,
}

impl Prediction {
    pub fn new(
        prediction_type: PredictionType,
        target: impl Into<String>,
        message: impl Into<String>,
        confidence: Confidence,
    ) -> Self {
        Self {
            prediction_type,
            target: target.into(),
            message: message.into(),
            confidence,
            days_until: None,
            preventive_action: None,
        }
    }

    pub fn with_days_until(mut self, days: u32) -> Self {
        self.days_until = Some(days);
        self
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.preventive_action = Some(action.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ORACLE
// ═══════════════════════════════════════════════════════════════════════════

/// Oráculo cuántico para predicción.
#[derive(Debug, Default)]
pub struct Oracle {
    /// Historial de cambios por documento.
    change_history: HashMap<String, Vec<u64>>,
    /// Umbral de días para stale detection.
    stale_threshold_days: u32,
}

impl Oracle {
    pub fn new() -> Self {
        Self {
            change_history: HashMap::new(),
            stale_threshold_days: 30,
        }
    }

    pub fn with_stale_threshold(mut self, days: u32) -> Self {
        self.stale_threshold_days = days;
        self
    }

    /// Predice errores potenciales en el proyecto.
    pub fn predict_errors(&self, _project: &ProjectState) -> Vec<Prediction> {
        let mut predictions = Vec::new();

        // Análisis de documentos por última actualización
        for doc in _project.documents.iter() {
            // Detectar documentos que pueden volverse stale
            if let Some(last_updated) = &doc.frontmatter.last_updated {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(last_updated, "%Y-%m-%d") {
                    let days_old = (chrono::Utc::now().date_naive() - date).num_days();

                    if days_old > self.stale_threshold_days as i64 {
                        predictions.push(
                            Prediction::new(
                                PredictionType::StaleDocument,
                                &doc.frontmatter.id,
                                format!("Documento sin actualizar hace {} días", days_old),
                                if days_old > 60 {
                                    Confidence::High
                                } else {
                                    Confidence::Medium
                                },
                            )
                            .with_action("Revisar y actualizar contenido"),
                        );
                    }
                }
            }

            // Detectar riesgo de links rotos
            if doc.links.len() > 10 {
                predictions.push(
                    Prediction::new(
                        PredictionType::BrokenLinkRisk,
                        &doc.frontmatter.id,
                        format!(
                            "Documento con {} enlaces, alto riesgo de roturas",
                            doc.links.len()
                        ),
                        Confidence::Low,
                    )
                    .with_action("Validar enlaces periódicamente"),
                );
            }
        }

        predictions
    }

    /// Sugiere acciones preventivas.
    pub fn suggest_preventive_actions(&self, predictions: &[Prediction]) -> Vec<String> {
        predictions
            .iter()
            .filter_map(|p| p.preventive_action.clone())
            .collect()
    }

    /// Registra un cambio para historial.
    pub fn record_change(&mut self, doc_id: &str, timestamp: u64) {
        self.change_history
            .entry(doc_id.to_string())
            .or_default()
            .push(timestamp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prediction_new() {
        let pred = Prediction::new(
            PredictionType::StaleDocument,
            "1.1",
            "Test prediction",
            Confidence::High,
        );

        assert_eq!(pred.confidence, Confidence::High);
        assert!(pred.preventive_action.is_none());
    }

    #[test]
    fn test_confidence_percent() {
        assert_eq!(Confidence::Low.as_percent(), 25);
        assert_eq!(Confidence::VeryHigh.as_percent(), 95);
    }

    #[test]
    fn test_oracle_new() {
        let oracle = Oracle::new().with_stale_threshold(60);
        assert_eq!(oracle.stale_threshold_days, 60);
    }

    #[test]
    fn test_record_change() {
        let mut oracle = Oracle::new();
        oracle.record_change("1.1", 1234567890);
        oracle.record_change("1.1", 1234567900);

        assert_eq!(oracle.change_history.get("1.1").unwrap().len(), 2);
    }
}
