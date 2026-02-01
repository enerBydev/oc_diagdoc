//! Trait para diagnóstico uniforme.

use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// DIAGNOSTIC SEVERITY
// ═══════════════════════════════════════════════════════════════════════════

/// Severidad de un diagnóstico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// Información.
    Info,
    /// Advertencia.
    Warning,
    /// Error.
    Error,
    /// Crítico.
    Critical,
}

impl DiagnosticSeverity {
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
            Self::Critical => "🚨",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DIAGNOSTIC
// ═══════════════════════════════════════════════════════════════════════════

/// Un diagnóstico individual.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Código único del diagnóstico.
    pub code: String,
    /// Severidad.
    pub severity: DiagnosticSeverity,
    /// Mensaje.
    pub message: String,
    /// Archivo afectado.
    pub file: Option<PathBuf>,
    /// Línea (si aplica).
    pub line: Option<usize>,
    /// Sugerencia de fix.
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn new(
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            file: None,
            line: None,
            suggestion: None,
        }
    }

    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Info, message)
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Warning, message)
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Error, message)
    }

    pub fn critical(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Critical, message)
    }

    pub fn with_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT DIAGNOSABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos que pueden diagnosticarse.
pub trait Diagnosable {
    /// Ejecuta diagnóstico y retorna lista de problemas.
    fn diagnose(&self) -> Vec<Diagnostic>;

    /// ¿Tiene errores?
    fn has_errors(&self) -> bool {
        self.diagnose()
            .iter()
            .any(|d| d.severity >= DiagnosticSeverity::Error)
    }

    /// ¿Tiene warnings o peor?
    fn has_warnings(&self) -> bool {
        self.diagnose()
            .iter()
            .any(|d| d.severity >= DiagnosticSeverity::Warning)
    }

    /// Cuenta diagnósticos por severidad.
    fn count_by_severity(&self) -> std::collections::HashMap<DiagnosticSeverity, usize> {
        let mut counts = std::collections::HashMap::new();
        for diag in self.diagnose() {
            *counts.entry(diag.severity).or_insert(0) += 1;
        }
        counts
    }

    /// Solo errores y críticos.
    fn errors(&self) -> Vec<Diagnostic> {
        self.diagnose()
            .into_iter()
            .filter(|d| d.severity >= DiagnosticSeverity::Error)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_new() {
        let diag = Diagnostic::error("E001", "Test error")
            .with_file("test.md")
            .with_suggestion("Fix it");

        assert_eq!(diag.code, "E001");
        assert_eq!(diag.severity, DiagnosticSeverity::Error);
        assert!(diag.suggestion.is_some());
    }

    #[test]
    fn test_severity_ordering() {
        assert!(DiagnosticSeverity::Critical > DiagnosticSeverity::Error);
        assert!(DiagnosticSeverity::Error > DiagnosticSeverity::Warning);
        assert!(DiagnosticSeverity::Warning > DiagnosticSeverity::Info);
    }

    #[test]
    fn test_severity_emoji() {
        assert_eq!(DiagnosticSeverity::Error.emoji(), "❌");
        assert_eq!(DiagnosticSeverity::Warning.emoji(), "⚠️");
    }
}
