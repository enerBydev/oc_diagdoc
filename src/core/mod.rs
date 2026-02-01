//! Módulo core del motor.

pub mod cli;
pub mod config;
pub mod docs;
pub mod files;
pub mod graph;
pub mod hash;
pub mod links;
pub mod loader;
pub mod pipeline;
pub mod registry;
pub mod release;
pub mod schema;
pub mod yaml;

pub use config::OcConfig;
pub use loader::{load_project, quick_stats};
