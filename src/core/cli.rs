//! Integración CLI completa.
//!
//! Punto de entrada y orquestación del CLI.

use clap::Parser;
use crate::DEFAULT_DATA_DIR;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// CLI TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Opciones globales del CLI.
#[derive(Parser, Debug, Clone)]
#[command(
    name = "oc_diagdoc",
    version = env!("CARGO_PKG_VERSION"),
    about = "Sistema de diagnóstico de documentación"
)]
pub struct CliApp {
    /// Modo verbose.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Modo silencioso.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Directorio de datos.
    #[arg(short, long, global = true, default_value = "Datos")]
    pub data_dir: PathBuf,

    /// Formato de salida (text/json/yaml).
    #[arg(long, global = true, default_value = "text")]
    pub format: String,

    /// Archivo de configuración.
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

impl CliApp {
    pub fn is_json(&self) -> bool {
        self.format == "json"
    }

    pub fn is_yaml(&self) -> bool {
        self.format == "yaml"
    }

    pub fn is_text(&self) -> bool {
        self.format == "text"
    }
}

impl Default for CliApp {
    fn default() -> Self {
        Self {
            verbose: false,
            quiet: false,
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            format: "text".to_string(),
            config: None,
        }
    }
}

/// Resultado de ejecución del CLI.
#[derive(Debug)]
pub struct CliResult {
    pub success: bool,
    pub exit_code: i32,
    pub message: Option<String>,
    pub duration_ms: u64,
}

impl CliResult {
    pub fn ok() -> Self {
        Self {
            success: true,
            exit_code: 0,
            message: None,
            duration_ms: 0,
        }
    }

    pub fn error(code: i32, msg: &str) -> Self {
        Self {
            success: false,
            exit_code: code,
            message: Some(msg.to_string()),
            duration_ms: 0,
        }
    }

    pub fn with_duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }
}

/// Contexto de ejecución.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub start_time: std::time::Instant,
    pub command: String,
    pub args: Vec<String>,
}

impl ExecutionContext {
    pub fn new(command: &str) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            command: command.to_string(),
            args: Vec::new(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_app_default() {
        let app = CliApp::default();
        assert!(!app.verbose);
        assert!(app.is_text());
    }

    #[test]
    fn test_cli_app_formats() {
        let mut app = CliApp::default();
        app.format = "json".to_string();
        assert!(app.is_json());
    }

    #[test]
    fn test_cli_result_ok() {
        let result = CliResult::ok();
        assert!(result.success);
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_cli_result_error() {
        let result = CliResult::error(1, "Failed");
        assert!(!result.success);
        assert_eq!(result.exit_code, 1);
    }

    #[test]
    fn test_execution_context() {
        let ctx = ExecutionContext::new("verify");
        assert_eq!(ctx.command, "verify");
    }
}
