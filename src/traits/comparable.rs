//! Trait para comparación semántica.
//!
//! Comparación que va más allá de igualdad simple.

use std::cmp::Ordering;

// ═══════════════════════════════════════════════════════════════════════════
// SIMILARITY SCORE
// ═══════════════════════════════════════════════════════════════════════════

/// Score de similitud entre 0.0 y 1.0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SimilarityScore(f64);

impl SimilarityScore {
    pub fn new(score: f64) -> Self {
        Self(score.clamp(0.0, 1.0))
    }
    
    pub fn exact() -> Self {
        Self(1.0)
    }
    
    pub fn none() -> Self {
        Self(0.0)
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
    
    pub fn is_exact(&self) -> bool {
        self.0 >= 0.999
    }
    
    pub fn is_similar(&self, threshold: f64) -> bool {
        self.0 >= threshold
    }
    
    pub fn as_percent(&self) -> f64 {
        self.0 * 100.0
    }
}

impl From<f64> for SimilarityScore {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT COMPARABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para comparación semántica.
pub trait Comparable<T = Self> {
    /// Calcula similitud con otro objeto.
    fn similarity(&self, other: &T) -> SimilarityScore;
    
    /// ¿Son semánticamente equivalentes?
    fn is_semantically_equal(&self, other: &T) -> bool {
        self.similarity(other).is_exact()
    }
    
    /// ¿Son similares dentro de un umbral?
    fn is_similar_to(&self, other: &T, threshold: f64) -> bool {
        self.similarity(other).is_similar(threshold)
    }
    
    /// Diferencias entre objetos.
    fn differences(&self, _other: &T) -> Vec<String> {
        Vec::new() // Default: sin diferencias detalladas
    }
}

/// Implementación para strings usando Levenshtein-like ratio.
impl Comparable for String {
    fn similarity(&self, other: &String) -> SimilarityScore {
        if self == other {
            return SimilarityScore::exact();
        }
        
        let len1 = self.len();
        let len2 = other.len();
        
        if len1 == 0 && len2 == 0 {
            return SimilarityScore::exact();
        }
        
        if len1 == 0 || len2 == 0 {
            return SimilarityScore::none();
        }
        
        // Simple character overlap ratio
        let common = self.chars()
            .filter(|c| other.contains(*c))
            .count();
        
        SimilarityScore::new(common as f64 / len1.max(len2) as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_score() {
        let score = SimilarityScore::new(0.85);
        assert!(score.is_similar(0.8));
        assert!(!score.is_exact());
    }

    #[test]
    fn test_string_similarity_exact() {
        let s1 = "hello".to_string();
        let s2 = "hello".to_string();
        
        assert!(s1.is_semantically_equal(&s2));
    }

    #[test]
    fn test_string_similarity_partial() {
        let s1 = "hello".to_string();
        let s2 = "helo".to_string();
        
        let sim = s1.similarity(&s2);
        assert!(sim.value() > 0.5);
    }
}
