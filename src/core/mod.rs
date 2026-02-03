//! # Módulo Core - Motor Algorítmico Nuclear
//!
//! Este módulo contiene los componentes fundamentales del motor `oc_diagdoc`.
//!
//! ## Submódulos
//!
//! | Módulo | Descripción |
//! |--------|-------------|
//! | [`cli`] | Parseador de argumentos CLI con clap |
//! | [`config`] | Configuración global del proyecto ([`OcConfig`]) |
//! | [`docs`] | Utilidades para manipulación de documentos |
//! | [`files`] | Sistema de archivos: escaneo, lectura, escritura atómica |
//! | [`graph`] | Grafo de dependencias y detección de ciclos |
//! | [`hash`] | Hashing SHA-256 con cache inteligente |
//! | [`links`] | Resolución de wiki-links `[[target]]` |
//! | [`loader`] | Cargador de proyectos completos |
//! | [`patterns`] | Patrones regex precompilados con Lazy |
//! | [`pipeline`] | Pipeline de procesamiento por etapas |
//! | [`registry`] | Registro de comandos disponibles |
//! | [`release`] | Información de versión y release |
//! | [`schema`] | Validación de frontmatter YAML |
//! | [`yaml`] | Parser de YAML con fallbacks |
//!
//! ## Uso básico
//!
//! ```rust,ignore
//! use oc_diagdoc_lib::core::{OcConfig, load_project};
//!
//! let config = OcConfig::default();
//! let project = load_project("Datos")?;
//! ```

pub mod cli;
pub mod config;
pub mod docs;
pub mod files;
pub mod graph;
pub mod hash;
pub mod links;
pub mod loader;
pub mod patterns;
pub mod pipeline;
pub mod registry;
pub mod release;
pub mod schema;
pub mod yaml;

pub use config::OcConfig;
pub use loader::{load_project, quick_stats};

