//! Tipos con lifetimes avanzados.
//!
//! Proporciona utilidades para manejo eficiente de referencias.

use std::borrow::Cow;

// ═══════════════════════════════════════════════════════════════════════════
// R39: BORROWED DOCUMENT VIEW
// ═══════════════════════════════════════════════════════════════════════════

/// Vista prestada de un documento (sin ownership).
#[derive(Debug, Clone)]
pub struct DocumentView<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}

impl<'a> DocumentView<'a> {
    pub fn new(id: &'a str, title: &'a str, content: &'a str) -> Self {
        Self { id, title, content }
    }
    
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R40: LAZY STRING (Cow)
// ═══════════════════════════════════════════════════════════════════════════

/// String perezoso que evita clonación innecesaria.
#[derive(Debug, Clone)]
pub struct LazyString<'a> {
    inner: Cow<'a, str>,
}

impl<'a> LazyString<'a> {
    pub fn borrowed(s: &'a str) -> Self {
        Self { inner: Cow::Borrowed(s) }
    }
    
    pub fn owned(s: String) -> Self {
        Self { inner: Cow::Owned(s) }
    }
    
    pub fn is_borrowed(&self) -> bool {
        matches!(self.inner, Cow::Borrowed(_))
    }
    
    pub fn is_owned(&self) -> bool {
        matches!(self.inner, Cow::Owned(_))
    }
    
    pub fn as_str(&self) -> &str {
        &self.inner
    }
    
    pub fn into_owned(self) -> String {
        self.inner.into_owned()
    }
    
    /// Modifica el string, convirtiendo a owned si es necesario.
    pub fn to_mut(&mut self) -> &mut String {
        self.inner.to_mut()
    }
}

impl<'a> From<&'a str> for LazyString<'a> {
    fn from(s: &'a str) -> Self {
        Self::borrowed(s)
    }
}

impl From<String> for LazyString<'static> {
    fn from(s: String) -> Self {
        Self::owned(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R41: CACHE ENTRY WITH LIFETIME
// ═══════════════════════════════════════════════════════════════════════════

/// Entrada de caché con referencia al valor.
#[derive(Debug)]
pub struct CacheEntry<'a, T> {
    pub key: &'a str,
    pub value: T,
    pub hits: usize,
}

impl<'a, T> CacheEntry<'a, T> {
    pub fn new(key: &'a str, value: T) -> Self {
        Self { key, value, hits: 0 }
    }
    
    pub fn hit(&mut self) {
        self.hits += 1;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R42: SPLIT BORROW PATTERN
// ═══════════════════════════════════════════════════════════════════════════

/// Estructura que permite borrowing parcial.
#[derive(Debug)]
pub struct SplitBorrow<'a, 'b> {
    pub metadata: &'a str,
    pub content: &'b str,
}

impl<'a, 'b> SplitBorrow<'a, 'b> {
    pub fn new(metadata: &'a str, content: &'b str) -> Self {
        Self { metadata, content }
    }
    
    /// Combina ambas partes.
    pub fn combined(&self) -> String {
        format!("{}\n---\n{}", self.metadata, self.content)
    }
}

/// Utilidad para split de frontmatter.
pub fn split_frontmatter(doc: &str) -> Option<SplitBorrow<'_, '_>> {
    let parts: Vec<&str> = doc.splitn(3, "---").collect();
    if parts.len() >= 3 {
        Some(SplitBorrow::new(parts[1].trim(), parts[2].trim()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_view() {
        let view = DocumentView::new("1.1", "Test", "hello world test");
        assert_eq!(view.word_count(), 3);
    }

    #[test]
    fn test_lazy_string_borrowed() {
        let s = "hello";
        let lazy = LazyString::borrowed(s);
        assert!(lazy.is_borrowed());
    }

    #[test]
    fn test_lazy_string_owned() {
        let lazy = LazyString::owned("hello".to_string());
        assert!(lazy.is_owned());
    }

    #[test]
    fn test_cache_entry() {
        let mut entry = CacheEntry::new("key", 42);
        entry.hit();
        entry.hit();
        assert_eq!(entry.hits, 2);
    }

    #[test]
    fn test_split_borrow() {
        let sb = SplitBorrow::new("meta", "content");
        assert!(sb.combined().contains("---"));
    }

    #[test]
    fn test_split_frontmatter() {
        let doc = "---\ntitle: Test\n---\nContent here";
        let result = split_frontmatter(doc);
        assert!(result.is_some());
    }
}
