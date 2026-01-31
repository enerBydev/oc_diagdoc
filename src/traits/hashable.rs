//! Trait para hashing consistente.
//!
//! Proporciona hashing determinístico para objetos.

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::types::ContentHash;

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT HASHABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos con hashing consistente.
pub trait Hashable {
    /// Genera contenido para hashear.
    fn hash_content(&self) -> Vec<u8>;
    
    /// Calcula hash del contenido.
    fn compute_hash(&self) -> ContentHash {
        ContentHash::from_bytes(&self.hash_content())
    }
    
    /// Verifica si el hash coincide.
    fn verify_hash(&self, expected: &ContentHash) -> bool {
        &self.compute_hash() == expected
    }
    
    /// Hash rápido como u64.
    fn quick_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash_content().hash(&mut hasher);
        hasher.finish()
    }
}

/// Implementación para strings.
impl Hashable for String {
    fn hash_content(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Hashable for str {
    fn hash_content(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Hashable for [u8] {
    fn hash_content(&self) -> Vec<u8> {
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hash() {
        let s = "hello".to_string();
        let hash = s.compute_hash();
        assert!(s.verify_hash(&hash));
    }

    #[test]
    fn test_quick_hash() {
        let s1 = "hello".to_string();
        let s2 = "hello".to_string();
        let s3 = "world".to_string();
        
        assert_eq!(s1.quick_hash(), s2.quick_hash());
        assert_ne!(s1.quick_hash(), s3.quick_hash());
    }

    #[test]
    fn test_hash_content() {
        let s = "test";
        let content = s.hash_content();
        assert_eq!(content, b"test".to_vec());
    }
}
