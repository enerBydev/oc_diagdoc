//! Implementaciones de AsRef, Deref y conversiones idiomáticas.
//!
//! Proporciona ergonomía para tipos personalizados.

use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::ops::Deref;

// ═══════════════════════════════════════════════════════════════════════════
// R23: DATAPATH TRAITS
// ═══════════════════════════════════════════════════════════════════════════

/// Wrapper sobre PathBuf con semántica enriquecida.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataPath {
    inner: PathBuf,
}

impl DataPath {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { inner: path.into() }
    }
    
    pub fn as_path(&self) -> &Path {
        &self.inner
    }
    
    pub fn to_path_buf(&self) -> PathBuf {
        self.inner.clone()
    }
}

impl Deref for DataPath {
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
        Self { inner: path.to_path_buf() }
    }
}

impl From<String> for DataPath {
    fn from(s: String) -> Self {
        Self { inner: PathBuf::from(s) }
    }
}

impl From<&str> for DataPath {
    fn from(s: &str) -> Self {
        Self { inner: PathBuf::from(s) }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R24: CONTENT HASH DEREF
// ═══════════════════════════════════════════════════════════════════════════

/// Wrapper para hash de contenido con Deref.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashString(String);

impl HashString {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }
}

impl Deref for HashString {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for HashString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for HashString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R25: DOCUMENT ID CONVERSIONS
// ═══════════════════════════════════════════════════════════════════════════

/// ID de documento con conversiones.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocId {
    parts: Vec<u32>,
    raw: String,
}

impl DocId {
    pub fn new(parts: Vec<u32>) -> Self {
        let raw = parts.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(".");
        Self { parts, raw }
    }
    
    pub fn parse(s: &str) -> Result<Self, String> {
        let parts: Result<Vec<u32>, _> = s.split('.')
            .map(|p| p.parse::<u32>())
            .collect();
        
        parts.map(Self::new)
            .map_err(|e| format!("Invalid document ID: {}", e))
    }
    
    pub fn module(&self) -> u32 {
        self.parts.first().copied().unwrap_or(0)
    }
    
    pub fn depth(&self) -> usize {
        self.parts.len()
    }
}

impl AsRef<str> for DocId {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

impl std::fmt::Display for DocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl From<DocId> for String {
    fn from(id: DocId) -> Self {
        id.raw
    }
}

impl TryFrom<&str> for DocId {
    type Error = String;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::parse(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R30-R32: ADDITIONAL ITERATORS
// ═══════════════════════════════════════════════════════════════════════════

/// Iterador que solo produce hojas (elementos sin hijos).
pub struct LeavesIter<I> {
    inner: I,
}

impl<I, T> LeavesIter<I>
where
    I: Iterator<Item = T>,
{
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I, T> Iterator for LeavesIter<I>
where
    I: Iterator<Item = T>,
    T: HasChildren,
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(item) if item.is_leaf() => return Some(item),
                Some(_) => continue,
                None => return None,
            }
        }
    }
}

/// Trait para elementos que pueden tener hijos.
pub trait HasChildren {
    fn is_leaf(&self) -> bool;
    fn child_count(&self) -> usize;
}

/// Iterador por módulo.
pub struct ModuleIter<I, T> {
    inner: I,
    module_id: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<I, T> ModuleIter<I, T>
where
    I: Iterator<Item = T>,
{
    pub fn new(inner: I, module_id: u32) -> Self {
        Self { 
            inner, 
            module_id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<I, T> Iterator for ModuleIter<I, T>
where
    I: Iterator<Item = T>,
    T: HasModule,
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(item) if item.module_id() == self.module_id => return Some(item),
                Some(_) => continue,
                None => return None,
            }
        }
    }
}

/// Trait para elementos con módulo.
pub trait HasModule {
    fn module_id(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_path_deref() {
        let dp = DataPath::new("/home/test");
        assert!(dp.is_absolute());
    }

    #[test]
    fn test_data_path_from() {
        let dp: DataPath = "/home/test".into();
        assert_eq!(dp.to_path_buf(), PathBuf::from("/home/test"));
    }

    #[test]
    fn test_hash_string_deref() {
        let hs = HashString::new("abc123");
        assert_eq!(&*hs, "abc123");
    }

    #[test]
    fn test_doc_id_parse() {
        let id = DocId::parse("1.2.3").unwrap();
        assert_eq!(id.module(), 1);
        assert_eq!(id.depth(), 3);
    }

    #[test]
    fn test_doc_id_display() {
        let id = DocId::new(vec![1, 2, 3]);
        assert_eq!(id.to_string(), "1.2.3");
    }

    #[test]
    fn test_doc_id_try_from() {
        let id: DocId = "5.10.15".try_into().unwrap();
        assert_eq!(id.module(), 5);
    }
}
