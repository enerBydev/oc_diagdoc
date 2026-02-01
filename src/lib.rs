//! oc_diagdoc_lib - Biblioteca del motor algorítmico nuclear
//!
//! Exporta todos los tipos y funcionalidades para uso como biblioteca.

pub mod commands;
pub mod core;
pub mod data;
pub mod errors;
pub mod quantum;
pub mod testing;
pub mod traits;
pub mod types;
pub mod ui;

#[macro_use]
pub mod macros;

// Re-exports principales
pub use core::config::OcConfig;
pub use data::document::Document;
pub use data::project::ProjectState;
pub use errors::{OcError, OcResult};
pub use types::{
    Breadcrumb, ContentHash, DocumentId, DocumentStatus, DocumentType, ModuleId, OcDate,
};

/// Configuración CLI común.
#[derive(Debug, Clone)]
pub struct CliConfig {
    /// Modo verbose
    pub verbose: bool,
    /// Directorio de datos
    pub data_dir: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            data_dir: "Datos".to_string(),
        }
    }
}
