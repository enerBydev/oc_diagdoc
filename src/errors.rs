//! Sistema de errores para oc_diagdoc.
//!
//! Define todos los tipos de errores posibles en el sistema.

use std::path::PathBuf;
use thiserror::Error;

/// Resultado estándar del sistema.
pub type OcResult<T> = Result<T, OcError>;

/// Error principal del sistema.
#[derive(Error, Debug)]
pub enum OcError {
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE PARSING
    // ═══════════════════════════════════════════════════════════════
    
    #[error("ID inválido: '{0}'")]
    InvalidId(String),
    
    #[error("Estado inválido: '{0}'")]
    InvalidStatus(String),
    
    #[error("Tipo de documento inválido: '{0}'")]
    InvalidDocType(String),
    
    #[error("Fecha inválida: '{0}'")]
    InvalidDate(String),
    
    #[error("Breadcrumb inválido: '{0}'")]
    InvalidBreadcrumb(String),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES YAML
    // ═══════════════════════════════════════════════════════════════
    
    #[error("YAML inválido en {path}: {message}")]
    YamlParse {
        path: PathBuf,
        message: String,
    },
    
    #[error("Frontmatter faltante en {0}")]
    MissingFrontmatter(PathBuf),
    
    #[error("Campo requerido faltante: '{field}' en {path}")]
    MissingField {
        field: String,
        path: PathBuf,
    },
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE FILESYSTEM
    // ═══════════════════════════════════════════════════════════════
    
    #[error("Error leyendo archivo {path}: {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Error escribiendo archivo {path}: {source}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Archivo no encontrado: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Directorio no encontrado: {0}")]
    DirectoryNotFound(PathBuf),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE VALIDACIÓN
    // ═══════════════════════════════════════════════════════════════
    
    #[error("Documento huérfano: {0}")]
    OrphanDocument(PathBuf),
    
    #[error("Error de validación: {message}")]
    Validation { message: String },
    
    #[error("Enlace roto: '{link}' en {file_path}")]
    BrokenLink {
        link: String,
        file_path: PathBuf,
    },
    
    #[error("Referencia circular detectada: {0}")]
    CircularReference(String),
    
    #[error("Duplicado detectado: {0}")]
    Duplicate(String),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE ESQUEMA
    // ═══════════════════════════════════════════════════════════════
    
    #[error("Violación de esquema: {0}")]
    SchemaViolation(String),
    
    #[error("Jerarquía inválida: {0}")]
    InvalidHierarchy(String),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE COMANDOS
    // ═══════════════════════════════════════════════════════════════
    
    #[error("Comando desconocido: {0}")]
    UnknownCommand(String),
    
    #[error("Argumento inválido: {0}")]
    InvalidArgument(String),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES DE CACHE
    // ═══════════════════════════════════════════════════════════════
    
    #[error("Error de cache: {0}")]
    CacheError(String),
    
    // ═══════════════════════════════════════════════════════════════
    // ERRORES GENÉRICOS
    // ═══════════════════════════════════════════════════════════════
    
    #[error("{0}")]
    Custom(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl OcError {
    /// Código de salida para CLI.
    pub fn exit_code(&self) -> i32 {
        match self {
            // Errores de usuario (1-9)
            Self::InvalidArgument(_) | Self::UnknownCommand(_) => 1,
            
            // Errores de archivo (10-19)
            Self::FileNotFound(_) | Self::DirectoryNotFound(_) => 10,
            Self::FileRead { .. } | Self::FileWrite { .. } => 11,
            
            // Errores de parsing (20-29)
            Self::InvalidId(_) | Self::InvalidStatus(_) | Self::InvalidDocType(_) => 20,
            Self::YamlParse { .. } | Self::MissingFrontmatter(_) | Self::MissingField { .. } => 21,
            
            // Errores de validación (30-39)
            Self::OrphanDocument(_) | Self::BrokenLink { .. } => 30,
            Self::CircularReference(_) | Self::Duplicate(_) => 31,
            Self::SchemaViolation(_) | Self::InvalidHierarchy(_) => 32,
            
            // Errores de sistema (90-99)
            Self::CacheError(_) => 90,
            
            // Otros
            _ => 99,
        }
    }
    
    /// ¿Es un error recuperable?
    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            Self::BrokenLink { .. } |
            Self::OrphanDocument(_) |
            Self::MissingField { .. }
        )
    }
}

/// Macro para crear OcError::Custom rápidamente.
#[macro_export]
macro_rules! oc_err {
    ($msg:expr) => {
        $crate::errors::OcError::Custom($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::errors::OcError::Custom(format!($fmt, $($arg)*))
    };
}

/// Macro para log + return error.
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::oc_err!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_exit_codes() {
        assert_eq!(OcError::InvalidArgument("test".into()).exit_code(), 1);
        assert_eq!(OcError::FileNotFound("test.md".into()).exit_code(), 10);
        assert_eq!(OcError::InvalidId("x".into()).exit_code(), 20);
    }
    
    #[test]
    fn test_error_display() {
        let err = OcError::InvalidId("3.x.1".to_string());
        assert!(err.to_string().contains("3.x.1"));
    }
}
