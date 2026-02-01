//! Identificadores únicos para documentos y módulos.

use crate::errors::OcError;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Identificador de documento jerárquico.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId {
    parts: Vec<u32>,
    raw: String,
}

impl DocumentId {
    /// Crea nuevo ID desde partes numéricas.
    pub fn new(parts: Vec<u32>) -> Self {
        let raw = parts
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(".");
        Self { parts, raw }
    }

    /// Retorna el módulo padre (primera parte).
    pub fn module(&self) -> u32 {
        self.parts.first().copied().unwrap_or(0)
    }

    /// Profundidad jerárquica (número de partes).
    pub fn depth(&self) -> usize {
        self.parts.len()
    }

    /// ¿Es un módulo padre (x.0)?
    pub fn is_module_root(&self) -> bool {
        self.parts.len() == 2 && self.parts[1] == 0
    }

    /// ¿Es el contextualizador?
    pub fn is_master(&self) -> bool {
        self.parts.len() == 1 && self.parts[0] == 0
    }

    /// ID del padre.
    pub fn parent(&self) -> Option<Self> {
        if self.parts.len() <= 1 {
            None
        } else {
            Some(Self::new(self.parts[..self.parts.len() - 1].to_vec()))
        }
    }

    /// Partes numéricas.
    pub fn parts(&self) -> &[u32] {
        &self.parts
    }

    /// String raw.
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl FromStr for DocumentId {
    type Err = OcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Result<Vec<u32>, _> = s.split('.').map(|p| p.parse::<u32>()).collect();

        match parts {
            Ok(p) if !p.is_empty() => Ok(Self {
                parts: p,
                raw: s.to_string(),
            }),
            _ => Err(OcError::InvalidId(s.to_string())),
        }
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Ord for DocumentId {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.parts.iter().zip(other.parts.iter()) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        self.parts.len().cmp(&other.parts.len())
    }
}

impl PartialOrd for DocumentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AsRef<str> for DocumentId {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

/// Identificador de módulo (simplificado).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub u32);

impl From<&DocumentId> for ModuleId {
    fn from(doc_id: &DocumentId) -> Self {
        Self(doc_id.module())
    }
}

impl From<u32> for ModuleId {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_id_parsing() {
        let id: DocumentId = "3.1.2".parse().unwrap();
        assert_eq!(id.module(), 3);
        assert_eq!(id.depth(), 3);
        assert!(!id.is_module_root());
    }

    #[test]
    fn test_module_root() {
        let id: DocumentId = "5.0".parse().unwrap();
        assert!(id.is_module_root());
    }

    #[test]
    fn test_ordering() {
        let a: DocumentId = "1.2.3".parse().unwrap();
        let b: DocumentId = "1.2.4".parse().unwrap();
        let c: DocumentId = "2.0".parse().unwrap();
        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn test_parent() {
        let id: DocumentId = "3.1.2".parse().unwrap();
        let parent = id.parent().unwrap();
        assert_eq!(parent.to_string(), "3.1");
    }
}
