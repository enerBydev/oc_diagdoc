//! Modelo de reporte de diagnóstico.
//!
//! Representa el resultado de un análisis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// REPORT SECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Sección de un reporte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    /// Título de la sección.
    pub title: String,
    /// Contenido.
    pub content: String,
    /// Subsecciones.
    pub subsections: Vec<ReportSection>,
}

impl ReportSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: String::new(),
            subsections: Vec::new(),
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn add_subsection(mut self, section: ReportSection) -> Self {
        self.subsections.push(section);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REPORT ITEM
// ═══════════════════════════════════════════════════════════════════════════

/// Severidad de un item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Un item individual del reporte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportItem {
    /// Tipo de diagnóstico.
    pub diagnostic_type: String,
    /// Archivo afectado.
    pub file: String,
    /// Mensaje.
    pub message: String,
    /// Severidad.
    pub severity: Severity,
    /// Sugerencia de fix.
    pub suggestion: Option<String>,
}

impl ReportItem {
    pub fn new(
        diagnostic_type: impl Into<String>,
        file: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            diagnostic_type: diagnostic_type.into(),
            file: file.into(),
            message: message.into(),
            severity: Severity::Info,
            suggestion: None,
        }
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REPORT
// ═══════════════════════════════════════════════════════════════════════════

/// Un reporte de diagnóstico completo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Título del reporte.
    pub title: String,
    /// Fecha de generación.
    pub generated_at: String,
    /// Resumen.
    pub summary: String,
    /// Secciones.
    pub sections: Vec<ReportSection>,
    /// Items del reporte.
    pub items: Vec<ReportItem>,
    /// Metadatos adicionales.
    pub metadata: HashMap<String, String>,
}

impl Report {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            summary: String::new(),
            sections: Vec::new(),
            items: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    pub fn add_section(mut self, section: ReportSection) -> Self {
        self.sections.push(section);
        self
    }

    pub fn add_item(mut self, item: ReportItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Cuenta items por severidad.
    pub fn count_by_severity(&self) -> HashMap<Severity, usize> {
        let mut counts = HashMap::new();
        for item in &self.items {
            *counts.entry(item.severity).or_insert(0) += 1;
        }
        counts
    }

    /// ¿Hay errores críticos?
    pub fn has_critical(&self) -> bool {
        self.items.iter().any(|i| i.severity == Severity::Critical)
    }

    /// ¿Hay errores?
    pub fn has_errors(&self) -> bool {
        self.items
            .iter()
            .any(|i| matches!(i.severity, Severity::Error | Severity::Critical))
    }

    /// Total de items.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// REPORT BUILDER
// ═══════════════════════════════════════════════════════════════════════════

/// Builder para reportes.
#[derive(Debug, Default)]
pub struct ReportBuilder {
    title: Option<String>,
    summary: Option<String>,
    sections: Vec<ReportSection>,
    items: Vec<ReportItem>,
    metadata: HashMap<String, String>,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn section(mut self, section: ReportSection) -> Self {
        self.sections.push(section);
        self
    }

    pub fn item(mut self, item: ReportItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Report {
        Report {
            title: self.title.unwrap_or_else(|| "Report".to_string()),
            generated_at: chrono::Utc::now().to_rfc3339(),
            summary: self.summary.unwrap_or_default(),
            sections: self.sections,
            items: self.items,
            metadata: self.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_builder() {
        let report = ReportBuilder::new()
            .title("Test Report")
            .summary("Test summary")
            .build();

        assert_eq!(report.title, "Test Report");
        assert!(report.items.is_empty());
    }

    #[test]
    fn test_report_items() {
        let report = Report::new("Test")
            .add_item(ReportItem::new("test", "file.md", "message").with_severity(Severity::Error))
            .add_item(ReportItem::new("test", "file2.md", "info"));

        assert_eq!(report.item_count(), 2);
        assert!(report.has_errors());
    }

    #[test]
    fn test_severity_count() {
        let report = Report::new("Test")
            .add_item(ReportItem::new("a", "", "").with_severity(Severity::Error))
            .add_item(ReportItem::new("b", "", "").with_severity(Severity::Error))
            .add_item(ReportItem::new("c", "", "").with_severity(Severity::Warning));

        let counts = report.count_by_severity();
        assert_eq!(counts.get(&Severity::Error), Some(&2));
        assert_eq!(counts.get(&Severity::Warning), Some(&1));
    }

    #[test]
    fn test_report_section() {
        let section = ReportSection::new("Title").with_content("Content here");

        assert_eq!(section.title, "Title");
        assert!(section.content.contains("Content"));
    }
}
