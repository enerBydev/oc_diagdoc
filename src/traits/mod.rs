//! Traits idiomáticos de Rust para oc_diagdoc.
//!
//! Define interfaces uniformes para:
//! - Validación (Validatable)
//! - Renderizado (Renderable)
//! - Cacheo (Cacheable)
//! - Diagnóstico (Diagnosable)
//! - Reparación (Fixable)
//! - Hashing (Hashable)
//! - Comparación (Comparable)
//! - Búsqueda (Searchable)
//! - Serialización (Serializable)
//! - Iteradores (PreOrderIter, PostOrderIter, LevelOrderIter)
//! - Conversiones (AsRef, Deref, From/Into)

pub mod validatable;
pub mod renderable;
pub mod cacheable;
pub mod diagnosable;
pub mod fixable;
pub mod hashable;
pub mod comparable;
pub mod searchable;
pub mod serializable;
pub mod iterators;
pub mod conversions;

pub use validatable::{Validatable, ValidatableCollection};
pub use renderable::{Renderable, OutputFormat};
pub use cacheable::Cacheable;
pub use diagnosable::{Diagnosable, Diagnostic, DiagnosticSeverity};
pub use fixable::{Fixable, Fix, FixResult};
pub use hashable::Hashable;
pub use comparable::{Comparable, SimilarityScore};
pub use searchable::{Searchable, SearchResult};
pub use serializable::{Serializable, Deserializable, SerializationFormat};
pub use iterators::{PreOrderIter, PostOrderIter, LevelOrderIter, FilteredIter, IteratorExt};
pub use conversions::{DataPath, HashString, DocId, HasChildren, HasModule, LeavesIter, ModuleIter};
