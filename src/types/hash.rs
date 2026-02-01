//! Hash de contenido para detección de cambios.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Hash SHA256 del contenido (primeros 16 chars).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ContentHash(String);

impl ContentHash {
    /// Calcula hash desde contenido.
    pub fn compute(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        Self(hex::encode(result))
    }

    /// Hash desde string existente.
    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    /// Hash vacío.
    pub fn empty() -> Self {
        Self(String::new())
    }

    /// Hash desde bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        Self(hex::encode(result))
    }

    /// Versión corta (primeros 8 chars).
    pub fn short(&self) -> &str {
        if self.0.len() >= 8 {
            &self.0[..8]
        } else {
            &self.0
        }
    }

    /// Hash completo.
    pub fn full(&self) -> &str {
        &self.0
    }

    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short())
    }
}

impl std::ops::Deref for ContentHash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ContentHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_compute() {
        let h = ContentHash::compute("hello world");
        assert_eq!(h.short().len(), 8);
        assert_eq!(h.full().len(), 64);
    }

    #[test]
    fn test_hash_consistency() {
        let h1 = ContentHash::compute("test");
        let h2 = ContentHash::compute("test");
        assert_eq!(h1, h2);
    }
}
