//! Comando sync - Sincronización de metadatos.
//!
//! Sincroniza fechas, hashes y metadatos entre documentos.

use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// SYNC RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Un cambio de sincronización.
#[derive(Debug, Clone)]
pub struct SyncChange {
    pub path: PathBuf,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

/// Resultado de sincronización.
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub changes: Vec<SyncChange>,
    pub files_scanned: usize,
    pub files_modified: usize,
}

impl SyncResult {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            files_scanned: 0,
            files_modified: 0,
        }
    }
    
    pub fn add_change(&mut self, change: SyncChange) {
        let path = change.path.clone();
        self.changes.push(change);
        
        // Contar archivos únicos modificados
        if !self.changes.iter()
            .take(self.changes.len() - 1)
            .any(|c| c.path == path) {
            self.files_modified += 1;
        }
    }
    
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }
}

impl Default for SyncResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SYNC COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de sincronización.
#[derive(Parser, Debug, Clone)]
#[command(name = "sync", about = "Sincronizar metadatos")]
pub struct SyncCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Sincronizar solo fechas.
    #[arg(long)]
    pub dates_only: bool,
    
    /// Sincronizar solo hashes.
    #[arg(long)]
    pub hashes_only: bool,
    
    /// Modo dry-run.
    #[arg(long)]
    pub dry_run: bool,
    
    /// Forzar actualización de todos.
    #[arg(long)]
    pub force: bool,
}

impl SyncCommand {
    pub fn run(&self) -> OcResult<SyncResult> {
        let mut result = SyncResult::new();
        
        // TODO: Implementar sincronización real
        
        Ok(result)
    }
    
    /// Genera timestamp actual ISO8601.
    pub fn current_timestamp() -> String {
        let now: DateTime<Utc> = SystemTime::now().into();
        now.format("%Y-%m-%dT%H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_result_new() {
        let result = SyncResult::new();
        assert!(!result.has_changes());
    }

    #[test]
    fn test_add_change() {
        let mut result = SyncResult::new();
        result.add_change(SyncChange {
            path: PathBuf::from("test.md"),
            field: "last_updated".to_string(),
            old_value: "2024-01-01".to_string(),
            new_value: "2024-01-30".to_string(),
        });
        
        assert!(result.has_changes());
        assert_eq!(result.files_modified, 1);
    }

    #[test]
    fn test_current_timestamp() {
        let ts = SyncCommand::current_timestamp();
        assert!(ts.contains("-"));
        assert!(ts.contains("T"));
    }

    #[test]
    fn test_multiple_changes_same_file() {
        let mut result = SyncResult::new();
        result.add_change(SyncChange {
            path: PathBuf::from("test.md"),
            field: "a".to_string(),
            old_value: "1".to_string(),
            new_value: "2".to_string(),
        });
        result.add_change(SyncChange {
            path: PathBuf::from("test.md"),
            field: "b".to_string(),
            old_value: "3".to_string(),
            new_value: "4".to_string(),
        });
        
        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.files_modified, 1);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: SyncCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    if cmd.dry_run {
        println!("🔍 Modo dry-run (sin cambios reales)");
    }
    
    if result.has_changes() {
        for change in &result.changes {
            println!("{}: {} → {}", 
                change.path.display(),
                change.old_value,
                change.new_value
            );
        }
        println!("\n🔄 {} cambios en {} archivos", 
            result.changes.len(), 
            result.files_modified
        );
    } else {
        println!("✅ Todo sincronizado, sin cambios necesarios");
    }
    
    Ok(())
}
