//! Path de datos con helpers.

use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Wrapper para paths con helpers de documentación.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataPath {
    inner: PathBuf,
}

impl DataPath {
    /// Crea nuevo DataPath.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { inner: path.into() }
    }

    /// Inner PathBuf.
    pub fn inner(&self) -> &PathBuf {
        &self.inner
    }

    /// ¿Es archivo Markdown?
    pub fn is_markdown(&self) -> bool {
        self.inner.extension().map(|e| e == "md").unwrap_or(false)
    }

    /// ¿Está en directorio Datos/?
    pub fn is_in_datos(&self) -> bool {
        self.inner.components().any(|c| c.as_os_str() == "Datos")
    }

    /// Nombre de archivo sin extensión.
    pub fn stem(&self) -> Option<&OsStr> {
        self.inner.file_stem()
    }

    /// Extrae ID del nombre de archivo (ej: "3.1.2 Título.md" -> "3.1.2").
    pub fn extract_id(&self) -> Option<String> {
        self.stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split_whitespace().next())
            .map(|s| s.to_string())
    }
}

impl std::ops::Deref for DataPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<Path> for DataPath {
    fn as_ref(&self) -> &Path {
        &self.inner
    }
}

impl AsRef<OsStr> for DataPath {
    fn as_ref(&self) -> &OsStr {
        self.inner.as_os_str()
    }
}

impl From<PathBuf> for DataPath {
    fn from(path: PathBuf) -> Self {
        Self { inner: path }
    }
}

impl From<&Path> for DataPath {
    fn from(path: &Path) -> Self {
        Self {
            inner: path.to_path_buf(),
        }
    }
}

impl From<String> for DataPath {
    fn from(s: String) -> Self {
        Self {
            inner: PathBuf::from(s),
        }
    }
}

impl From<&str> for DataPath {
    fn from(s: &str) -> Self {
        Self {
            inner: PathBuf::from(s),
        }
    }
}

impl std::fmt::Display for DataPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_id() {
        let p = DataPath::new("Datos/Módulo 3/3.1.2 Título.md");
        assert_eq!(p.extract_id(), Some("3.1.2".to_string()));
    }

    #[test]
    fn test_is_markdown() {
        let p = DataPath::new("test.md");
        assert!(p.is_markdown());

        let p = DataPath::new("test.txt");
        assert!(!p.is_markdown());
    }
}
