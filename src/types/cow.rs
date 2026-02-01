//! Optimizaciones Cow (Copy-on-Write).
//!
//! Minimiza clonaciones mediante semántica Cow.

use std::borrow::Cow;
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════════════════
// R43: COW STRING UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

/// String inteligente que evita clonación cuando no es necesaria.
#[derive(Debug, Clone)]
pub struct SmartString<'a> {
    inner: Cow<'a, str>,
}

impl<'a> SmartString<'a> {
    pub fn borrowed(s: &'a str) -> Self {
        Self {
            inner: Cow::Borrowed(s),
        }
    }

    pub fn owned(s: String) -> Self {
        Self {
            inner: Cow::Owned(s),
        }
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.inner, Cow::Borrowed(_))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Normaliza espacios solo si es necesario.
    pub fn normalize_whitespace(&mut self) {
        if self.inner.contains("  ") || self.inner.contains('\t') {
            let normalized: String = self.inner.split_whitespace().collect::<Vec<_>>().join(" ");
            self.inner = Cow::Owned(normalized);
        }
    }

    /// Convierte a minúsculas solo si es necesario.
    pub fn to_lowercase(&mut self) {
        if self.inner.chars().any(|c| c.is_uppercase()) {
            self.inner = Cow::Owned(self.inner.to_lowercase());
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn into_owned(self) -> String {
        self.inner.into_owned()
    }
}

impl<'a> From<&'a str> for SmartString<'a> {
    fn from(s: &'a str) -> Self {
        Self::borrowed(s)
    }
}

impl From<String> for SmartString<'static> {
    fn from(s: String) -> Self {
        Self::owned(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R44: COW PATH UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

/// Path inteligente con semántica Cow.
#[derive(Debug, Clone)]
pub struct SmartPath<'a> {
    inner: Cow<'a, Path>,
}

impl<'a> SmartPath<'a> {
    pub fn borrowed(path: &'a Path) -> Self {
        Self {
            inner: Cow::Borrowed(path),
        }
    }

    pub fn owned(path: PathBuf) -> Self {
        Self {
            inner: Cow::Owned(path),
        }
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.inner, Cow::Borrowed(_))
    }

    /// Une un componente, clonando solo si es necesario.
    pub fn join(&self, component: &str) -> SmartPath<'static> {
        SmartPath::owned(self.inner.join(component))
    }

    /// Normaliza el path solo si es necesario.
    pub fn normalize(&mut self) {
        let normalized: PathBuf = self.inner.components().collect();
        if normalized != self.inner.as_ref() {
            self.inner = Cow::Owned(normalized);
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    pub fn exists(&self) -> bool {
        self.inner.exists()
    }
}

impl<'a> From<&'a Path> for SmartPath<'a> {
    fn from(path: &'a Path) -> Self {
        Self::borrowed(path)
    }
}

impl From<PathBuf> for SmartPath<'static> {
    fn from(path: PathBuf) -> Self {
        Self::owned(path)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R45: COW COLLECTION UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

/// Vector inteligente que evita clonación.
#[derive(Debug, Clone)]
pub struct SmartVec<'a, T: Clone> {
    inner: Cow<'a, [T]>,
}

impl<'a, T: Clone> SmartVec<'a, T> {
    pub fn borrowed(slice: &'a [T]) -> Self {
        Self {
            inner: Cow::Borrowed(slice),
        }
    }

    pub fn owned(vec: Vec<T>) -> Self {
        Self {
            inner: Cow::Owned(vec),
        }
    }

    pub fn is_borrowed(&self) -> bool {
        matches!(self.inner, Cow::Borrowed(_))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Push que clona solo si es necesario.
    pub fn push(&mut self, value: T) {
        self.inner.to_mut().push(value);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn into_owned(self) -> Vec<T> {
        self.inner.into_owned()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SmartVec<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self::borrowed(slice)
    }
}

impl<T: Clone> From<Vec<T>> for SmartVec<'static, T> {
    fn from(vec: Vec<T>) -> Self {
        Self::owned(vec)
    }
}

/// Optimizado: procesa collection sin clonar si no hay modificación.
pub fn process_without_clone<'a, T, F>(items: &'a [T], predicate: F) -> Cow<'a, [T]>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    if items.iter().all(|item| predicate(item)) {
        Cow::Borrowed(items)
    } else {
        Cow::Owned(
            items
                .iter()
                .filter(|item| predicate(item))
                .cloned()
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_string_borrowed() {
        let s = "hello";
        let smart = SmartString::borrowed(s);
        assert!(smart.is_borrowed());
    }

    #[test]
    fn test_smart_string_normalize() {
        let mut smart = SmartString::owned("hello   world".to_string());
        smart.normalize_whitespace();
        assert_eq!(smart.as_str(), "hello world");
    }

    #[test]
    fn test_smart_path_borrowed() {
        let path = Path::new("/test/path");
        let smart = SmartPath::borrowed(path);
        assert!(smart.is_borrowed());
    }

    #[test]
    fn test_smart_path_join() {
        let path = Path::new("/test");
        let smart = SmartPath::borrowed(path);
        let joined = smart.join("file.md");
        assert!(!joined.is_borrowed());
    }

    #[test]
    fn test_smart_vec_borrowed() {
        let vec = vec![1, 2, 3];
        let smart: SmartVec<i32> = SmartVec::borrowed(&vec);
        assert!(smart.is_borrowed());
    }

    #[test]
    fn test_smart_vec_push() {
        let mut smart: SmartVec<i32> = SmartVec::owned(vec![1, 2]);
        smart.push(3);
        assert_eq!(smart.len(), 3);
    }

    #[test]
    fn test_process_without_clone_no_filter() {
        let items = vec![1, 2, 3];
        let result = process_without_clone(&items, |_| true);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_process_without_clone_with_filter() {
        let items = vec![1, 2, 3, 4, 5];
        let result = process_without_clone(&items, |&x| x > 2);
        assert!(matches!(result, Cow::Owned(_)));
    }
}
