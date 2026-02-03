//! # oc_diagdoc_lib - Motor Algorítmico Nuclear para Documentación
//!
// Clippy allows para warnings aceptables que no afectan correctitud:
#![allow(clippy::ptr_arg)]               // &PathBuf es aceptable en APIs públicas
#![allow(clippy::needless_borrow)]       // borrowed expression aceptable
#![allow(clippy::manual_strip)]          // strip_prefix manual aceptable por legibilidad
#![allow(clippy::new_ret_no_self)]       // Error::other pendiente de stabilization
#![allow(clippy::field_reassign_with_default)]  // Default::default() + campos es válido
#![allow(clippy::derivable_impls)]       // Implementaciones manuales válidas
#![allow(clippy::should_implement_trait)] // from_str personalizado válido
#![allow(clippy::redundant_closure)]     // Closures explícitas más claras
#![allow(clippy::collapsible_else_if)]   // else if separado más legible
#![allow(clippy::to_string_in_format_args)] // .to_string() en format explícito es ok
#![allow(clippy::too_many_arguments)]    // Funciones con muchos args son intencionales
#![allow(clippy::search_is_some)]        // .find().is_none() es legible
#![allow(clippy::double_ended_iterator_last)] // .last() es más legible que .next_back()
#![allow(clippy::expect_fun_call)]       // expect con fn call aceptable
#![allow(clippy::cmp_owned)]             // owned comparison aceptable para edge cases
#![allow(clippy::manual_is_ascii_check)] // char comparison manual más clara
#![allow(clippy::consecutive_str_replace)]  // replace en cadena legible
#![allow(clippy::io_other_error)]         // std::io::Error::other pendiente
//!
//! Biblioteca Rust de alto rendimiento para gestión, validación y análisis
//! de documentación estructurada en formato Markdown con frontmatter YAML.
//!
//! ## Características principales
//!
//! - 🔍 **Verificación**: 21 fases de validación automática
//! - 📊 **Estadísticas**: Métricas detalladas por módulo
//! - 🔗 **Links**: Resolución y validación de wiki-links
//! - 🌳 **Árbol**: Visualización jerárquica de documentos
//! - ⚡ **Performance**: Compilado a código nativo, <100ms típico
//!
//! ## Arquitectura de módulos
//!
//! ```text
//! oc_diagdoc_lib
//! ├── core/       # Motor algorítmico central
//! ├── commands/   # Implementación de comandos CLI
//! ├── data/       # Estructuras de datos (Document, Project)
//! ├── types/      # Tipos fundamentales (DocumentId, OcDate)
//! ├── traits/     # Traits compartidos (Validatable, Queryable)
//! ├── errors/     # Sistema de errores tipado (OcError)
//! ├── ui/         # Interfaz de usuario (tablas, colores)
//! └── quantum/    # Algoritmos de optimización avanzada
//! ```
//!
//! ## Uso básico
//!
//! ```rust,ignore
//! use oc_diagdoc_lib::{OcConfig, Document, OcResult};
//! use oc_diagdoc_lib::core::load_project;
//!
//! fn main() -> OcResult<()> {
//!     let project = load_project("Datos")?;
//!     println!("Documentos: {}", project.document_count());
//!     Ok(())
//! }
//! ```
//!
//! ## Versión
//!
//! - **Versión**: 3.0.1
//! - **Rust Edition**: 2021
//! - **MSRV**: 1.70+

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

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTES GLOBALES
// ═══════════════════════════════════════════════════════════════════════════

/// Directorio de datos por defecto.
/// Usado como default en todos los comandos CLI.
pub const DEFAULT_DATA_DIR: &str = "Datos";

/// Configuración CLI común para todos los comandos.
///
/// Esta estructura contiene los parámetros globales que se pasan
/// a cada comando desde la línea de comandos.
///
/// # Ejemplo
///
/// ```rust,ignore
/// let config = CliConfig {
///     verbose: true,
///     data_dir: DEFAULT_DATA_DIR.to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CliConfig {
    /// Modo verbose - muestra información adicional de debug
    pub verbose: bool,
    /// Modo quiet - suprime output no esencial
    pub quiet: bool,
    /// Directorio de datos donde residen los documentos Markdown
    pub data_dir: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            quiet: false,
            data_dir: DEFAULT_DATA_DIR.to_string(),
        }
    }
}

