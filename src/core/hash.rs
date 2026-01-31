//! Motor de hashing para detección de cambios en contenido.
//!
//! Proporciona:
//! - Cálculo de hashes SHA256 para contenido y archivos
//! - Cache de hashes con invalidación por tiempo de modificación

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use sha2::{Sha256, Digest};
use parking_lot::RwLock;
use crate::errors::{OcError, OcResult};
use crate::types::ContentHash;
use crate::core::files::{read_file_content, get_file_metadata};

/// Calcula hash SHA256 de un contenido string.
pub fn compute_content_hash(content: &str) -> ContentHash {
    ContentHash::compute(content)
}

/// Calcula hash SHA256 de un archivo.
pub fn compute_file_hash(path: impl AsRef<Path>) -> OcResult<ContentHash> {
    let content = read_file_content(path)?;
    Ok(compute_content_hash(&content))
}

/// Calcula hash de múltiples archivos combinados.
pub fn compute_multi_file_hash(paths: &[PathBuf]) -> OcResult<ContentHash> {
    let mut hasher = Sha256::new();
    
    for path in paths {
        let content = read_file_content(path)?;
        hasher.update(content.as_bytes());
        hasher.update(b"\0"); // Separador
    }
    
    let result = hasher.finalize();
    Ok(ContentHash::from_string(hex::encode(result)))
}

/// Entrada en el cache de hashes.
#[derive(Debug, Clone)]
struct HashCacheEntry {
    hash: ContentHash,
    modified_time: SystemTime,
}

/// Cache de hashes de archivos con invalidación automática.
#[derive(Debug, Default)]
pub struct HashCache {
    cache: RwLock<HashMap<PathBuf, HashCacheEntry>>,
}

impl HashCache {
    /// Crea un nuevo cache vacío.
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }
    
    /// Obtiene hash del cache o lo calcula si no existe/está stale.
    pub fn get_or_compute(&self, path: impl AsRef<Path>) -> OcResult<ContentHash> {
        let path = path.as_ref();
        let metadata = get_file_metadata(path)?;
        
        // Verificar cache
        {
            let cache = self.cache.read();
            if let Some(entry) = cache.get(path) {
                if entry.modified_time == metadata.modified {
                    return Ok(entry.hash.clone());
                }
            }
        }
        
        // Calcular nuevo hash
        let hash = compute_file_hash(path)?;
        
        // Guardar en cache
        {
            let mut cache = self.cache.write();
            cache.insert(path.to_path_buf(), HashCacheEntry {
                hash: hash.clone(),
                modified_time: metadata.modified,
            });
        }
        
        Ok(hash)
    }
    
    /// Invalida una entrada del cache.
    pub fn invalidate(&self, path: impl AsRef<Path>) {
        let mut cache = self.cache.write();
        cache.remove(path.as_ref());
    }
    
    /// Invalida todas las entradas del cache.
    pub fn clear(&self) {
        let mut cache = self.cache.write();
        cache.clear();
    }
    
    /// Número de entradas en cache.
    pub fn len(&self) -> usize {
        let cache = self.cache.read();
        cache.len()
    }
    
    /// ¿Cache vacío?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Verifica si un archivo ha cambiado comparando con hash almacenado.
    pub fn has_changed(&self, path: impl AsRef<Path>) -> OcResult<bool> {
        let path = path.as_ref();
        let metadata = get_file_metadata(path)?;
        
        let cache = self.cache.read();
        if let Some(entry) = cache.get(path) {
            Ok(entry.modified_time != metadata.modified)
        } else {
            Ok(true) // No está en cache, asumimos que cambió
        }
    }
    
    /// Pre-carga hashes de múltiples archivos.
    pub fn preload(&self, paths: &[PathBuf]) -> OcResult<usize> {
        let mut loaded = 0;
        for path in paths {
            if self.get_or_compute(path).is_ok() {
                loaded += 1;
            }
        }
        Ok(loaded)
    }
    
    /// Limpia entradas stale (archivos que ya no existen).
    pub fn cleanup_stale(&self) -> usize {
        let mut cache = self.cache.write();
        let before = cache.len();
        cache.retain(|path, _| path.exists());
        before - cache.len()
    }
}

/// Verifica si el contenido de un archivo coincide con un hash.
pub fn verify_content_hash(path: impl AsRef<Path>, expected: &ContentHash) -> OcResult<bool> {
    let actual = compute_file_hash(path)?;
    Ok(&actual == expected)
}

/// Compara hashes de dos archivos.
pub fn files_have_same_content(path1: impl AsRef<Path>, path2: impl AsRef<Path>) -> OcResult<bool> {
    let hash1 = compute_file_hash(path1)?;
    let hash2 = compute_file_hash(path2)?;
    Ok(hash1 == hash2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_content_hash() {
        let content = "Hello, World!";
        let hash1 = compute_content_hash(content);
        let hash2 = compute_content_hash(content);
        
        assert_eq!(hash1, hash2);
        assert!(!hash1.full().is_empty());
    }

    #[test]
    fn test_file_hash() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        
        fs::write(&file_path, "Test content").unwrap();
        
        let hash = compute_file_hash(&file_path).unwrap();
        assert!(!hash.full().is_empty());
    }

    #[test]
    fn test_hash_cache_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("cached.md");
        
        fs::write(&file_path, "Cached content").unwrap();
        
        let cache = HashCache::new();
        
        // Primera vez: calcula
        let hash1 = cache.get_or_compute(&file_path).unwrap();
        assert_eq!(cache.len(), 1);
        
        // Segunda vez: usa cache
        let hash2 = cache.get_or_compute(&file_path).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_cache_invalidation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalidate.md");
        
        fs::write(&file_path, "Original").unwrap();
        
        let cache = HashCache::new();
        let _hash1 = cache.get_or_compute(&file_path).unwrap();
        assert_eq!(cache.len(), 1);
        
        // Invalidar
        cache.invalidate(&file_path);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_hash_cache_stale_detection() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("stale.md");
        
        fs::write(&file_path, "Original content").unwrap();
        
        let cache = HashCache::new();
        let hash1 = cache.get_or_compute(&file_path).unwrap();
        
        // Esperar un momento y modificar
        thread::sleep(Duration::from_millis(100));
        fs::write(&file_path, "Modified content").unwrap();
        
        // El cache detecta el cambio y recalcula
        let hash2 = cache.get_or_compute(&file_path).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_content_hash() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("verify.md");
        
        let content = "Verify this content";
        fs::write(&file_path, content).unwrap();
        
        let hash = compute_content_hash(content);
        assert!(verify_content_hash(&file_path, &hash).unwrap());
        
        // Hash diferente
        let wrong_hash = compute_content_hash("Different content");
        assert!(!verify_content_hash(&file_path, &wrong_hash).unwrap());
    }
}
