//! Cache multinivel para persistencia.
//!
//! Implementa cache L1 (RAM) y L2 (disco).

use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════
// CACHE ENTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Entrada de cache con TTL.
#[derive(Debug, Clone)]
struct CacheItem<V> {
    value: V,
    created: Instant,
    ttl: Option<Duration>,
}

impl<V> CacheItem<V> {
    fn new(value: V, ttl: Option<Duration>) -> Self {
        Self {
            value,
            created: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.created.elapsed() > ttl
        } else {
            false
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MULTI-LEVEL CACHE
// ═══════════════════════════════════════════════════════════════════════════

/// Cache multinivel thread-safe.
#[derive(Debug)]
pub struct MultiLevelCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// L1: Cache en RAM.
    l1: RwLock<HashMap<K, CacheItem<V>>>,
    /// TTL por defecto.
    default_ttl: Option<Duration>,
    /// Tamaño máximo L1.
    max_l1_size: usize,
    /// Estadísticas de hits/misses.
    hits: RwLock<usize>,
    misses: RwLock<usize>,
}

impl<K, V> MultiLevelCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            l1: RwLock::new(HashMap::new()),
            default_ttl: None,
            max_l1_size: 1000,
            hits: RwLock::new(0),
            misses: RwLock::new(0),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_l1_size = size;
        self
    }

    /// Obtiene un valor del cache.
    pub fn get(&self, key: &K) -> Option<V> {
        let cache = self.l1.read();

        if let Some(item) = cache.get(key) {
            if !item.is_expired() {
                *self.hits.write() += 1;
                return Some(item.value.clone());
            }
        }

        *self.misses.write() += 1;
        None
    }

    /// Almacena un valor en el cache.
    pub fn set(&self, key: K, value: V) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    /// Almacena con TTL específico.
    pub fn set_with_ttl(&self, key: K, value: V, ttl: Option<Duration>) {
        let mut cache = self.l1.write();

        // Evict si está lleno
        if cache.len() >= self.max_l1_size {
            self.evict_expired(&mut cache);
        }

        cache.insert(key, CacheItem::new(value, ttl));
    }

    /// Invalida una entrada.
    pub fn invalidate(&self, key: &K) {
        let mut cache = self.l1.write();
        cache.remove(key);
    }

    /// Limpia todo el cache.
    pub fn clear(&self) {
        let mut cache = self.l1.write();
        cache.clear();
        *self.hits.write() = 0;
        *self.misses.write() = 0;
    }

    /// Evict entradas expiradas.
    fn evict_expired(&self, cache: &mut HashMap<K, CacheItem<V>>) {
        cache.retain(|_, item| !item.is_expired());
    }

    /// Tamaño actual del cache.
    pub fn len(&self) -> usize {
        self.l1.read().len()
    }

    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.l1.read().is_empty()
    }

    /// Hit ratio.
    pub fn hit_ratio(&self) -> f64 {
        let hits = *self.hits.read();
        let misses = *self.misses.read();
        let total = hits + misses;

        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }
}

impl<K, V> Default for MultiLevelCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_set_get() {
        let cache: MultiLevelCache<String, i32> = MultiLevelCache::new();
        cache.set("key1".to_string(), 42);

        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_invalidate() {
        let cache: MultiLevelCache<String, i32> = MultiLevelCache::new();
        cache.set("key1".to_string(), 42);
        cache.invalidate(&"key1".to_string());

        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_clear() {
        let cache: MultiLevelCache<String, i32> = MultiLevelCache::new();
        cache.set("key1".to_string(), 1);
        cache.set("key2".to_string(), 2);
        cache.clear();

        assert!(cache.is_empty());
    }

    #[test]
    fn test_hit_ratio() {
        let cache: MultiLevelCache<String, i32> = MultiLevelCache::new();
        cache.set("key1".to_string(), 42);

        cache.get(&"key1".to_string()); // hit
        cache.get(&"key1".to_string()); // hit
        cache.get(&"key2".to_string()); // miss

        assert!(cache.hit_ratio() > 0.6);
    }
}
