//! Módulo de testing.
//!
//! Fixtures, mocks y utilidades para tests.

pub mod fixtures;
pub mod mocks;

pub use fixtures::{TestProject, sample_frontmatter, minimal_document, generate_module_docs};
pub use fixtures::{assert_file_exists, assert_file_contains};
pub use mocks::{MockFileSystem, MockConfig, MockLogger, MockCache};
