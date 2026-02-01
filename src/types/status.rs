//! Estados de documentos.

use crate::errors::OcError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Estado de un documento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DocumentStatus {
    /// En desarrollo activo.
    Active,
    /// Borrador sin revisar.
    #[default]
    Draft,
    /// Revisado y aprobado.
    Reviewed,
    /// Obsoleto, no usar.
    Deprecated,
    /// Archivado.
    Archived,
    /// Pendiente de contenido.
    Stub,
}

impl DocumentStatus {
    /// ¿Es un estado saludable?
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Active | Self::Reviewed)
    }

    /// Código de color para UI.
    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Active => "green",
            Self::Draft => "yellow",
            Self::Reviewed => "cyan",
            Self::Deprecated => "red",
            Self::Archived => "dim",
            Self::Stub => "magenta",
        }
    }

    /// Emoji representativo.
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Active => "🟢",
            Self::Draft => "🟡",
            Self::Reviewed => "✅",
            Self::Deprecated => "💀",
            Self::Archived => "📦",
            Self::Stub => "📝",
        }
    }

    /// Todos los estados posibles.
    pub fn all() -> &'static [Self] {
        &[
            Self::Active,
            Self::Draft,
            Self::Reviewed,
            Self::Deprecated,
            Self::Archived,
            Self::Stub,
        ]
    }
}

impl FromStr for DocumentStatus {
    type Err = OcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" | "activo" => Ok(Self::Active),
            "draft" | "borrador" => Ok(Self::Draft),
            "reviewed" | "revisado" => Ok(Self::Reviewed),
            "deprecated" | "obsoleto" => Ok(Self::Deprecated),
            "archived" | "archivado" => Ok(Self::Archived),
            "stub" | "pendiente" => Ok(Self::Stub),
            _ => Err(OcError::InvalidStatus(s.to_string())),
        }
    }
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Active => "active",
            Self::Draft => "draft",
            Self::Reviewed => "reviewed",
            Self::Deprecated => "deprecated",
            Self::Archived => "archived",
            Self::Stub => "stub",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_parsing() {
        assert_eq!(
            "active".parse::<DocumentStatus>().unwrap(),
            DocumentStatus::Active
        );
        assert_eq!(
            "activo".parse::<DocumentStatus>().unwrap(),
            DocumentStatus::Active
        );
    }

    #[test]
    fn test_healthy() {
        assert!(DocumentStatus::Active.is_healthy());
        assert!(DocumentStatus::Reviewed.is_healthy());
        assert!(!DocumentStatus::Draft.is_healthy());
    }
}
