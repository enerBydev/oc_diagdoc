//! Módulo core del motor.

pub mod config;
pub mod yaml;
pub mod files;
pub mod hash;
pub mod links;
pub mod graph;
pub mod schema;
pub mod pipeline;
pub mod registry;
pub mod cli;
pub mod docs;
pub mod release;
pub mod loader;

pub use config::OcConfig;
pub use loader::{load_project, quick_stats};
