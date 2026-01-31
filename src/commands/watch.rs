//! Comando watch - Observar cambios en tiempo real.
//!
//! Monitorea cambios en la documentación y ejecuta acciones.

use std::path::PathBuf;
use std::time::Duration;
use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// WATCH TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Evento de cambio detectado.
#[derive(Debug, Clone)]
pub struct WatchEvent {
    pub path: PathBuf,
    pub event_type: WatchEventType,
    pub timestamp: String,
}

/// Tipo de evento.
#[derive(Debug, Clone, PartialEq)]
pub enum WatchEventType {
    Created,
    Modified,
    Deleted,
    Renamed,
}

/// Configuración del watcher.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub debounce_ms: u64,
    pub patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 500,
            patterns: vec!["*.md".to_string()],
            ignore_patterns: vec!["node_modules".to_string(), ".git".to_string()],
        }
    }
}

impl WatchConfig {
    pub fn debounce_duration(&self) -> Duration {
        Duration::from_millis(self.debounce_ms)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WATCH COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando watch.
#[derive(Parser, Debug, Clone)]
#[command(name = "watch", about = "Observar cambios")]
pub struct WatchCommand {
    /// Ruta a observar.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Comando a ejecutar en cambios.
    #[arg(short, long)]
    pub exec: Option<String>,
    
    /// Debounce en ms.
    #[arg(long, default_value = "500")]
    pub debounce: u64,
    
    /// Modo silencioso.
    #[arg(short, long)]
    pub quiet: bool,
}

impl WatchCommand {
    pub fn run(&self) -> OcResult<()> {
        // TODO: Implementar watcher real con notify crate
        println!("👁️  Observando cambios... (Ctrl+C para salir)");
        Ok(())
    }
    
    pub fn config(&self) -> WatchConfig {
        WatchConfig {
            debounce_ms: self.debounce,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert_eq!(config.debounce_ms, 500);
        assert!(config.patterns.contains(&"*.md".to_string()));
    }

    #[test]
    fn test_debounce_duration() {
        let config = WatchConfig {
            debounce_ms: 1000,
            ..Default::default()
        };
        
        assert_eq!(config.debounce_duration(), Duration::from_millis(1000));
    }

    #[test]
    fn test_watch_event_type() {
        let event = WatchEvent {
            path: PathBuf::from("test.md"),
            event_type: WatchEventType::Modified,
            timestamp: "2024-01-30".to_string(),
        };
        
        assert_eq!(event.event_type, WatchEventType::Modified);
    }

    #[test]
    fn test_watch_command_config() {
        let cmd = WatchCommand {
            path: None,
            exec: None,
            debounce: 1000,
            quiet: false,
        };
        
        assert_eq!(cmd.config().debounce_ms, 1000);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: WatchCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let config = cmd.config();
    
    if !cmd.quiet {
        println!("👁️  Observando: {:?}", cmd.path.as_deref().unwrap_or(std::path::Path::new(".")));
        println!("⚡ Debounce: {}ms", config.debounce_ms);
        if let Some(exec) = &cmd.exec {
            println!("🔧 Ejecutando: {}", exec);
        }
    }
    
    cmd.run()?;
    
    Ok(())
}
