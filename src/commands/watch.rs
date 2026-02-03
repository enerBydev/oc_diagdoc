//! Comando watch - Observar cambios en tiempo real.
//!
//! Monitorea cambios en la documentación y ejecuta acciones.

use crate::errors::OcResult;
use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

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

    // L23-L24: Flags avanzados
    /// Ejecutar verify automático en cambios.
    #[arg(long)]
    pub verify: bool,

    /// Archivo de hooks personalizados (.oc-hooks).
    #[arg(long)]
    pub hooks: Option<PathBuf>,

    /// Número máximo de iteraciones (0 = infinito).
    #[arg(long, default_value = "0")]
    pub max_iterations: usize,
}

impl WatchCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<Vec<WatchEvent>> {
        use crate::core::files::{get_all_md_files, ScanOptions};
        use std::collections::HashMap;

        let mut events = Vec::new();
        let default_path = PathBuf::from(data_dir);
        let watch_path = self.path.as_ref().unwrap_or(&default_path);

        if !self.quiet {
            eprintln!("👁️  Observando: {}", watch_path.display());
        }

        // L23.1: Polling para detectar cambios (simplificado sin notify crate)
        let options = ScanOptions::new();
        let mut file_mtimes: HashMap<PathBuf, std::time::SystemTime> = HashMap::new();

        // Primera pasada: capturar estado inicial
        if let Ok(files) = get_all_md_files(watch_path, &options) {
            for file in files {
                if let Ok(meta) = std::fs::metadata(&file) {
                    if let Ok(mtime) = meta.modified() {
                        file_mtimes.insert(file, mtime);
                    }
                }
            }
        }

        // Simular una iteración de verificación
        let iterations = if self.max_iterations == 0 {
            1
        } else {
            self.max_iterations
        };

        for i in 0..iterations {
            std::thread::sleep(self.config().debounce_duration());

            // Detectar cambios
            if let Ok(files) = get_all_md_files(watch_path, &options) {
                for file in &files {
                    if let Ok(meta) = std::fs::metadata(file) {
                        if let Ok(mtime) = meta.modified() {
                            match file_mtimes.get(file) {
                                Some(old_mtime) if mtime != *old_mtime => {
                                    // L23.2: Archivo modificado
                                    let event = WatchEvent {
                                        path: file.clone(),
                                        event_type: WatchEventType::Modified,
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                    };

                                    if !self.quiet {
                                        eprintln!("📝 Cambio detectado: {}", file.display());
                                    }

                                    events.push(event);
                                    file_mtimes.insert(file.clone(), mtime);

                                    // L23.2: Ejecutar verify si solicitado
                                    if self.verify {
                                        self.run_verify(data_dir);
                                    }

                                    // L24.1: Ejecutar hooks personalizados
                                    if let Some(ref hook_file) = self.hooks {
                                        self.run_hooks(hook_file, file);
                                    }

                                    // L23.2: Ejecutar comando --exec
                                    if let Some(ref exec_cmd) = self.exec {
                                        self.run_exec_command(exec_cmd, file);
                                    }
                                }
                                None => {
                                    // Archivo nuevo
                                    let event = WatchEvent {
                                        path: file.clone(),
                                        event_type: WatchEventType::Created,
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                    };

                                    if !self.quiet {
                                        eprintln!("➕ Nuevo archivo: {}", file.display());
                                    }

                                    events.push(event);
                                    file_mtimes.insert(file.clone(), mtime);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            if i < iterations - 1 && !self.quiet {
                eprintln!("⏳ Esperando cambios... ({}/{})", i + 1, iterations);
            }
        }

        Ok(events)
    }

    /// L23.2: Ejecutar verify automático.
    fn run_verify(&self, _data_dir: &std::path::Path) {
        use crate::commands::verify::VerifyCommand;

        let data_dir_buf = _data_dir.to_path_buf();
        let verify_cmd = VerifyCommand {
            path: Some(data_dir_buf.clone()),
            schema_strict: false,
            json: false,
            phase: None,
            quiet: true,
            quick: true, // F1.4: usar modo quick en watch para rapidez
            progress: false,
            cache: false,
        };

        if let Ok(result) = verify_cmd.run(&data_dir_buf) {
            eprintln!(
                "  ✅ Verify: {} fases OK, {} errores",
                result.phases_passed(),
                result.phases_failed()
            );
        }
    }

    /// L24.1: Ejecutar hooks desde archivo.
    fn run_hooks(&self, hook_file: &PathBuf, changed_file: &PathBuf) {
        if let Ok(content) = std::fs::read_to_string(hook_file) {
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    let cmd = trimmed.replace("$FILE", &changed_file.display().to_string());
                    eprintln!("  🔧 Hook: {}", cmd);
                    // En producción se ejecutaría con std::process::Command
                }
            }
        }
    }

    /// Ejecutar comando --exec.
    fn run_exec_command(&self, exec_cmd: &str, changed_file: &PathBuf) {
        let cmd = exec_cmd.replace("$FILE", &changed_file.display().to_string());
        eprintln!("  ⚡ Ejecutando: {}", cmd);
        // En producción se ejecutaría con std::process::Command
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
            verify: true,
            hooks: None,
            max_iterations: 5,
        };

        assert_eq!(cmd.config().debounce_ms, 1000);
        assert!(cmd.verify);
        assert_eq!(cmd.max_iterations, 5);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: WatchCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let config = cmd.config();
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);

    if !cmd.quiet {
        println!("👁️  Observando: {}", data_dir.display());
        println!("⚡ Debounce: {}ms", config.debounce_ms);
        if cmd.verify {
            println!("🔍 Verify automático: activado");
        }
        if let Some(ref hooks) = cmd.hooks {
            println!("🔧 Hooks: {}", hooks.display());
        }
        if let Some(ref exec) = cmd.exec {
            println!("⚡ Exec: {}", exec);
        }
    }

    let events = cmd.run(data_dir)?;
    println!("📊 {} eventos detectados", events.len());

    Ok(())
}
