//! Trait para búsqueda en contenido.
//!
//! Permite buscar dentro de objetos.

// ═══════════════════════════════════════════════════════════════════════════
// SEARCH RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de búsqueda.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Posición del match.
    pub position: usize,
    /// Longitud del match.
    pub length: usize,
    /// Contexto alrededor del match.
    pub context: Option<String>,
    /// Score de relevancia (0.0 - 1.0).
    pub relevance: f64,
}

impl SearchResult {
    pub fn new(position: usize, length: usize) -> Self {
        Self {
            position,
            length,
            context: None,
            relevance: 1.0,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn with_relevance(mut self, relevance: f64) -> Self {
        self.relevance = relevance.clamp(0.0, 1.0);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT SEARCHABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos que pueden ser buscados.
pub trait Searchable {
    /// Busca un patrón.
    fn search(&self, pattern: &str) -> Vec<SearchResult>;

    /// ¿Contiene el patrón?
    fn contains_pattern(&self, pattern: &str) -> bool {
        !self.search(pattern).is_empty()
    }

    /// Cuenta ocurrencias.
    fn count_matches(&self, pattern: &str) -> usize {
        self.search(pattern).len()
    }

    /// Primera ocurrencia.
    fn find_first(&self, pattern: &str) -> Option<SearchResult> {
        self.search(pattern).into_iter().next()
    }

    /// Busca case-insensitive.
    fn search_ignore_case(&self, pattern: &str) -> Vec<SearchResult>;
}

/// Implementación para String.
impl Searchable for String {
    fn search(&self, pattern: &str) -> Vec<SearchResult> {
        self.match_indices(pattern)
            .map(|(pos, matched)| {
                let context_start = pos.saturating_sub(20);
                let context_end = (pos + matched.len() + 20).min(self.len());
                let context = self[context_start..context_end].to_string();

                SearchResult::new(pos, matched.len()).with_context(context)
            })
            .collect()
    }

    fn search_ignore_case(&self, pattern: &str) -> Vec<SearchResult> {
        let lower_self = self.to_lowercase();
        let lower_pattern = pattern.to_lowercase();

        lower_self
            .match_indices(&lower_pattern)
            .map(|(pos, matched)| SearchResult::new(pos, matched.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let text = "hello world hello".to_string();
        let results = text.search("hello");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].position, 0);
        assert_eq!(results[1].position, 12);
    }

    #[test]
    fn test_contains_pattern() {
        let text = "foo bar baz".to_string();
        assert!(text.contains_pattern("bar"));
        assert!(!text.contains_pattern("qux"));
    }

    #[test]
    fn test_search_ignore_case() {
        let text = "Hello HELLO hello".to_string();
        let results = text.search_ignore_case("hello");

        assert_eq!(results.len(), 3);
    }
}
