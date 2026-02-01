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

pub mod cacheable;
pub mod comparable;
pub mod conversions;
pub mod diagnosable;
pub mod fixable;
pub mod hashable;
pub mod iterators;
pub mod renderable;
pub mod searchable;
pub mod serializable;
pub mod validatable;

pub use cacheable::Cacheable;
pub use comparable::{Comparable, SimilarityScore};
pub use conversions::{
    DataPath, DocId, HasChildren, HasModule, HashString, LeavesIter, ModuleIter,
};
pub use diagnosable::{Diagnosable, Diagnostic, DiagnosticSeverity};
pub use fixable::{Fix, FixResult, Fixable};
pub use hashable::Hashable;
pub use iterators::{FilteredIter, IteratorExt, LevelOrderIter, PostOrderIter, PreOrderIter};
pub use renderable::{OutputFormat, Renderable};
pub use searchable::{SearchResult, Searchable};
pub use serializable::{Deserializable, Serializable, SerializationFormat};
pub use validatable::{Validatable, ValidatableCollection};
