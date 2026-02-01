//! Mocks para testing.
//!
//! Implementaciones mock de traits para tests.

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// MOCK TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Mock de sistema de archivos.
#[derive(Debug, Default)]
pub struct MockFileSystem {
    files: HashMap<String, String>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: &str, content: &str) {
        self.files.insert(path.to_string(), content.to_string());
    }

    pub fn read(&self, path: &str) -> Option<&String> {
        self.files.get(path)
    }

    pub fn exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// Mock de configuración.
#[derive(Debug, Clone)]
pub struct MockConfig {
    pub values: HashMap<String, String>,
}

impl MockConfig {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
}

impl Default for MockConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock de logger.
#[derive(Debug, Default)]
pub struct MockLogger {
    pub messages: Vec<(String, String)>, // (level, message)
}

impl MockLogger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(&mut self, level: &str, msg: &str) {
        self.messages.push((level.to_string(), msg.to_string()));
    }

    pub fn info(&mut self, msg: &str) {
        self.log("INFO", msg);
    }

    pub fn error(&mut self, msg: &str) {
        self.log("ERROR", msg);
    }

    pub fn has_errors(&self) -> bool {
        self.messages.iter().any(|(l, _)| l == "ERROR")
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// Mock de cache.
#[derive(Debug, Default)]
pub struct MockCache<V> {
    data: HashMap<String, V>,
    hits: usize,
    misses: usize,
}

impl<V: Clone> MockCache<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<V> {
        match self.data.get(key) {
            Some(v) => {
                self.hits += 1;
                Some(v.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    pub fn set(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_filesystem() {
        let mut fs = MockFileSystem::new();
        fs.add_file("/test.md", "content");
        assert!(fs.exists("/test.md"));
    }

    #[test]
    fn test_mock_config() {
        let mut config = MockConfig::new();
        config.set("key", "value");
        assert_eq!(config.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_mock_logger() {
        let mut logger = MockLogger::new();
        logger.error("test error");
        assert!(logger.has_errors());
    }

    #[test]
    fn test_mock_cache() {
        let mut cache: MockCache<i32> = MockCache::new();
        cache.set("key", 42);
        assert_eq!(cache.get("key"), Some(42));
    }
}
