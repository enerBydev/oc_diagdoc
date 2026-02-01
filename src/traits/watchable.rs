//! Trait Watchable - Para observación de cambios en documentos.

use std::time::SystemTime;

/// Evento de cambio.
#[derive(Debug, Clone)]
pub struct ChangeEvent {
    pub path: String,
    pub change_type: ChangeType,
    pub timestamp: SystemTime,
}

/// Tipo de cambio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Renamed { from: String, to: String },
}

impl ChangeEvent {
    pub fn created(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            change_type: ChangeType::Created,
            timestamp: SystemTime::now(),
        }
    }
    
    pub fn modified(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            change_type: ChangeType::Modified,
            timestamp: SystemTime::now(),
        }
    }
    
    pub fn deleted(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            change_type: ChangeType::Deleted,
            timestamp: SystemTime::now(),
        }
    }
}

/// Trait para elementos que pueden ser observados.
pub trait Watchable {
    /// Ruta del elemento.
    fn watch_path(&self) -> &str;
    
    /// Última modificación.
    fn last_modified(&self) -> Option<SystemTime>;
    
    /// ¿Ha sido modificado desde un tiempo dado?
    fn modified_since(&self, since: SystemTime) -> bool {
        self.last_modified()
            .map(|t| t > since)
            .unwrap_or(false)
    }
    
    /// Hash del contenido para detectar cambios.
    fn content_hash(&self) -> u64;
    
    /// ¿El contenido ha cambiado respecto a un hash previo?
    fn content_changed(&self, previous_hash: u64) -> bool {
        self.content_hash() != previous_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_change_event_created() {
        let event = ChangeEvent::created("test.md");
        assert_eq!(event.path, "test.md");
        assert_eq!(event.change_type, ChangeType::Created);
    }
    
    #[test]
    fn test_change_type() {
        let renamed = ChangeType::Renamed {
            from: "old.md".to_string(),
            to: "new.md".to_string(),
        };
        assert!(matches!(renamed, ChangeType::Renamed { .. }));
    }
}
