//! # Tipos Fundamentales
//!
//! Tipos de datos atómicos utilizados en todo el sistema oc_diagdoc.
//!
//! ## Tipos principales
//!
//! | Tipo | Descripción |
//! |------|-------------|
//! | [`DocumentId`] | Identificador jerárquico (ej: "1.2.3") |
//! | [`ModuleId`] | Identificador de módulo (primer nivel del ID) |
//! | [`OcDate`] | Fecha con parsing flexible (ISO/RFC3339) |
//! | [`DocumentStatus`] | Estado del documento (activo, borrador, etc.) |
//! | [`DocumentType`] | Tipo de documento (hoja, sección, módulo) |
//! | [`Breadcrumb`] | Ruta de navegación jerárquica |
//! | [`ContentHash`] | Hash SHA-256 del contenido |
//!
//! ## Tipos avanzados
//!
//! | Tipo | Descripción |
//! |------|-------------|
//! | [`SmartString`] | Copy-on-write string optimizado |
//! | [`SmartPath`] | Path con cow semántica |
//! | [`CacheEntry`] | Entrada de caché con lifetime |
//! | [`DocumentView`] | Vista inmutable de documento |
//!
//! ## Ejemplo
//!
//! ```rust,ignore
//! use oc_diagdoc_lib::types::{DocumentId, DocumentStatus};
//!
//! let id: DocumentId = "2.5.1".parse()?;
//! let status = DocumentStatus::Active;
//! ```

pub mod breadcrumb;
pub mod cow;
pub mod date;
pub mod doc_type;
pub mod hash;
pub mod id;
pub mod lifetimes;
pub mod metrics;
pub mod path;
pub mod status;
pub mod severity;  // ADD#2

// Re-exports
pub use breadcrumb::Breadcrumb;
pub use cow::{SmartPath, SmartString, SmartVec};
pub use date::OcDate;
pub use doc_type::DocumentType;
pub use hash::ContentHash;
pub use id::{DocumentId, ModuleId};
pub use lifetimes::{CacheEntry, DocumentView, LazyString, SplitBorrow};
pub use metrics::{Counters, CoverageStats};
pub use path::DataPath;
pub use status::DocumentStatus;
pub use severity::{Severity, Issue};  // ADD#2

