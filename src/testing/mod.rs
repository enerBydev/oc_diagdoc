//! Módulo de testing.
//!
//! Fixtures, mocks y utilidades para tests.

pub mod fixtures;
pub mod mocks;

pub use fixtures::{assert_file_contains, assert_file_exists};
pub use fixtures::{generate_module_docs, minimal_document, sample_frontmatter, TestProject};
pub use mocks::{MockCache, MockConfig, MockFileSystem, MockLogger};
