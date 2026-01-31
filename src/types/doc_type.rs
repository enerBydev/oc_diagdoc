//! Tipos de documentos en la jerarquía.

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::errors::OcError;

/// Tipo de documento según su posición jerárquica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    /// Contextualizador (ID=0).
    Master,
    /// Nodo principal de módulo (x.0).
    ModuleRoot,
    /// Rama intermedia.
    Branch,
    /// Documento final sin hijos.
    Leaf,
}

impl DocumentType {
    /// ¿Puede tener hijos?
    pub fn can_have_children(&self) -> bool {
        matches!(self, Self::Master | Self::ModuleRoot | Self::Branch)
    }

    /// Emoji representativo.
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Master => "👑",
            Self::ModuleRoot => "📁",
            Self::Branch => "📂",
            Self::Leaf => "📄",
        }
    }
}

impl FromStr for DocumentType {
    type Err = OcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "master" | "contextualizador" => Ok(Self::Master),
            "moduleroot" | "module_root" | "modulo" => Ok(Self::ModuleRoot),
            "branch" | "rama" => Ok(Self::Branch),
            "leaf" | "hoja" => Ok(Self::Leaf),
            _ => Err(OcError::InvalidDocType(s.to_string())),
        }
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Master => "master",
            Self::ModuleRoot => "module_root",
            Self::Branch => "branch",
            Self::Leaf => "leaf",
        };
        write!(f, "{}", s)
    }
}
