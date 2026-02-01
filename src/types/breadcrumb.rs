//! Breadcrumb jerárquico.

use serde::{Deserialize, Serialize};

/// Breadcrumb como lista de segmentos.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Breadcrumb {
    segments: Vec<String>,
    raw: String,
}

impl Breadcrumb {
    /// Crea desde string separado por ">".
    pub fn parse(s: &str) -> Self {
        let segments: Vec<String> = s
            .split('>')
            .map(|seg| seg.trim().to_string())
            .filter(|seg| !seg.is_empty())
            .collect();

        Self {
            raw: s.to_string(),
            segments,
        }
    }

    /// Número de segmentos.
    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// Segmentos individuales.
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Último segmento (título actual).
    pub fn current(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Primer segmento (raíz).
    pub fn root(&self) -> Option<&str> {
        self.segments.first().map(|s| s.as_str())
    }
}

impl std::fmt::Display for Breadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl From<&str> for Breadcrumb {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for Breadcrumb {
    fn from(s: String) -> Self {
        Self::parse(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_parse() {
        let bc = Breadcrumb::parse("OnlyCar > Módulo 3 > Subpágina");
        assert_eq!(bc.depth(), 3);
        assert_eq!(bc.current(), Some("Subpágina"));
        assert_eq!(bc.root(), Some("OnlyCar"));
    }
}
