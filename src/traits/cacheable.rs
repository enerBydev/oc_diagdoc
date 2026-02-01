//! Trait para caching y memoización.

use crate::types::ContentHash;
use std::hash::Hash;

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT CACHEABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos que pueden ser cacheados.
pub trait Cacheable {
    /// Tipo de clave para el cache.
    type Key: Hash + Eq + Clone;

    /// Genera la clave de cache.
    fn cache_key(&self) -> Self::Key;

    /// Hash del contenido para invalidación.
    fn content_hash(&self) -> ContentHash;

    /// ¿Ha cambiado desde el último hash conocido?
    fn has_changed(&self, previous_hash: &ContentHash) -> bool {
        &self.content_hash() != previous_hash
    }

    /// Tiempo de vida sugerido del cache en segundos.
    fn cache_ttl(&self) -> Option<u64> {
        None // Sin expiración por defecto
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CACHE ENTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Entrada de cache con metadatos.
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    /// Valor cacheado.
    pub value: T,
    /// Hash del contenido cuando se cacheó.
    pub hash: ContentHash,
    /// Timestamp de cuando se cacheó.
    pub cached_at: std::time::Instant,
    /// TTL opcional.
    pub ttl: Option<std::time::Duration>,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T, hash: ContentHash) -> Self {
        Self {
            value,
            hash,
            cached_at: std::time::Instant::now(),
            ttl: None,
        }
    }

    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// ¿Ha expirado?
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.cached_at.elapsed() > ttl
        } else {
            false
        }
    }

    /// ¿Es válido para el hash dado?
    pub fn is_valid_for(&self, current_hash: &ContentHash) -> bool {
        !self.is_expired() && &self.hash == current_hash
    }

    /// Edad del cache.
    pub fn age(&self) -> std::time::Duration {
        self.cached_at.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache_entry_new() {
        let hash = ContentHash::compute("test");
        let entry = CacheEntry::new("value", hash.clone());

        assert_eq!(entry.value, "value");
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_is_valid_for() {
        let hash = ContentHash::compute("test");
        let entry = CacheEntry::new("value", hash.clone());

        assert!(entry.is_valid_for(&hash));

        let other_hash = ContentHash::compute("other");
        assert!(!entry.is_valid_for(&other_hash));
    }

    #[test]
    fn test_cache_entry_with_ttl() {
        let hash = ContentHash::compute("test");
        let entry = CacheEntry::new("value", hash).with_ttl(Duration::from_secs(3600));

        assert!(!entry.is_expired());
    }
}
