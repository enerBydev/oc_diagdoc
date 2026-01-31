//! oc_diagdoc_lib - Biblioteca del motor algorítmico nuclear
//!
//! Exporta todos los tipos y funcionalidades para uso como biblioteca.

pub mod errors;
pub mod types;
pub mod core;
pub mod data;
pub mod quantum;
pub mod ui;
pub mod traits;
pub mod commands;
pub mod testing;

#[macro_use]
pub mod macros;

// Re-exports principales
pub use errors::{OcError, OcResult};
pub use types::{DocumentId, ModuleId, DocumentStatus, DocumentType, Breadcrumb, OcDate, ContentHash};
pub use core::config::OcConfig;
pub use data::document::Document;
pub use data::project::ProjectState;

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
