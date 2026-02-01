//! Módulo de tipos base.

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
