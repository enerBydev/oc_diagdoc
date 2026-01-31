//! Módulo de tipos base.

pub mod id;
pub mod status;
pub mod doc_type;
pub mod breadcrumb;
pub mod date;
pub mod hash;
pub mod path;
pub mod metrics;
pub mod lifetimes;
pub mod cow;

// Re-exports
pub use id::{DocumentId, ModuleId};
pub use status::DocumentStatus;
pub use doc_type::DocumentType;
pub use breadcrumb::Breadcrumb;
pub use date::OcDate;
pub use hash::ContentHash;
pub use path::DataPath;
pub use metrics::{Counters, CoverageStats};
pub use lifetimes::{DocumentView, LazyString, CacheEntry, SplitBorrow};
pub use cow::{SmartString, SmartPath, SmartVec};
