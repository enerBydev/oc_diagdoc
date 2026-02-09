//! Módulo de tipos de severidad para issues
//!
//! ADD#2: Sistema de severidad configurable

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Nivel de severidad de un issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Error crítico que debe resolverse
    Error = 3,
    /// Advertencia que debería revisarse
    Warning = 2,
    /// Información útil
    Info = 1,
    /// Sugerencia de mejora
    Hint = 0,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl Severity {
    /// Emoji/icon para el nivel de severidad
    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Error => "❌",
            Severity::Warning => "⚠️",
            Severity::Info => "ℹ️",
            Severity::Hint => "💡",
        }
    }

    /// Color asociado al nivel de severidad
    pub fn color(&self) -> Color {
        match self {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
            Severity::Info => Color::Blue,
            Severity::Hint => Color::Gray,
        }
    }

    /// ANSI escape code para terminal
    pub fn ansi_color(&self) -> &'static str {
        match self {
            Severity::Error => "\x1b[31m",    // Red
            Severity::Warning => "\x1b[33m",  // Yellow
            Severity::Info => "\x1b[34m",     // Blue
            Severity::Hint => "\x1b[90m",     // Gray
        }
    }

    /// Nombre en formato corto
    pub fn short_name(&self) -> &'static str {
        match self {
            Severity::Error => "ERR",
            Severity::Warning => "WARN",
            Severity::Info => "INFO",
            Severity::Hint => "HINT",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" | "err" | "e" => Some(Severity::Error),
            "warning" | "warn" | "w" => Some(Severity::Warning),
            "info" | "i" => Some(Severity::Info),
            "hint" | "h" => Some(Severity::Hint),
            _ => None,
        }
    }

    /// Todos los niveles ordenados por severidad (mayor a menor)
    pub fn all() -> Vec<Self> {
        vec![Severity::Error, Severity::Warning, Severity::Info, Severity::Hint]
    }

    /// Verifica si este nivel cumple con el mínimo requerido
    pub fn meets_minimum(&self, min: Severity) -> bool {
        *self >= min
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Info => "Info",
            Severity::Hint => "Hint",
        })
    }
}

/// Issue con severidad y metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// ID único del issue
    pub id: String,
    /// Mensaje descriptivo
    pub message: String,
    /// Nivel de severidad
    pub severity: Severity,
    /// Si es corregible automáticamente
    pub fixable: bool,
    /// Fase de verificación (1-21)
    pub phase: u8,
    /// Archivo relacionado (opcional)
    pub file: Option<String>,
    /// Línea del archivo (opcional)
    pub line: Option<usize>,
    /// Código de regla (opcional)
    pub rule_code: Option<String>,
}

impl Issue {
    /// Crea un nuevo issue de error
    pub fn error(phase: u8, message: impl Into<String>) -> Self {
        Self {
            id: format!("E{:02}", phase),
            message: message.into(),
            severity: Severity::Error,
            fixable: false,
            phase,
            file: None,
            line: None,
            rule_code: None,
        }
    }

    /// Crea un nuevo issue de warning
    pub fn warning(phase: u8, message: impl Into<String>) -> Self {
        Self {
            id: format!("W{:02}", phase),
            message: message.into(),
            severity: Severity::Warning,
            fixable: true,
            phase,
            file: None,
            line: None,
            rule_code: None,
        }
    }

    /// Crea un nuevo issue informativo
    pub fn info(phase: u8, message: impl Into<String>) -> Self {
        Self {
            id: format!("I{:02}", phase),
            message: message.into(),
            severity: Severity::Info,
            fixable: false,
            phase,
            file: None,
            line: None,
            rule_code: None,
        }
    }

    /// Formato compacto para output
    pub fn compact(&self) -> String {
        format!("{} [{}] {}", self.severity.icon(), self.id, self.message)
    }

    /// Formato con color ANSI
    pub fn colored(&self) -> String {
        format!("{}{} [{}] {}\x1b[0m", 
            self.severity.ansi_color(), 
            self.severity.icon(), 
            self.id, 
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Warning > Severity::Info);
        assert!(Severity::Info > Severity::Hint);
    }

    #[test]
    fn test_severity_from_str() {
        assert_eq!(Severity::from_str("error"), Some(Severity::Error));
        assert_eq!(Severity::from_str("WARN"), Some(Severity::Warning));
        assert_eq!(Severity::from_str("hint"), Some(Severity::Hint));
        assert_eq!(Severity::from_str("invalid"), None);
    }

    #[test]
    fn test_issue_creation() {
        let err = Issue::error(1, "Test error");
        assert_eq!(err.severity, Severity::Error);
        assert_eq!(err.id, "E01");
        assert!(!err.fixable);
    }

    #[test]
    fn test_meets_minimum() {
        assert!(Severity::Error.meets_minimum(Severity::Warning));
        assert!(Severity::Warning.meets_minimum(Severity::Warning));
        assert!(!Severity::Info.meets_minimum(Severity::Warning));
    }
}
