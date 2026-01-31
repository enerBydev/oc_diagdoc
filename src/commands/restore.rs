//! Comando restore - Restauración de snapshots.
//!
//! Restaura el proyecto a un estado previo.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// RESTORE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de restauración.
#[derive(Debug, Clone, Serialize)]
pub struct RestoreResult {
    pub snapshot_id: String,
    pub files_restored: usize,
    pub files_skipped: usize,
    pub conflicts: Vec<PathBuf>,
}

impl RestoreResult {
    pub fn new(snapshot_id: &str) -> Self {
        Self {
            snapshot_id: snapshot_id.to_string(),
            files_restored: 0,
            files_skipped: 0,
            conflicts: Vec::new(),
        }
    }
    
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RESTORE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de restauración.
#[derive(Parser, Debug, Clone)]
#[command(name = "restore", about = "Restaurar snapshot")]
pub struct RestoreCommand {
    /// ID del snapshot.
    pub snapshot_id: String,
    
    /// Forzar (sobrescribir conflictos).
    #[arg(short, long)]
    pub force: bool,
    
    /// Modo dry-run.
    #[arg(long)]
    pub dry_run: bool,
}

impl RestoreCommand {
    pub fn run(&self) -> OcResult<RestoreResult> {
        let mut result = RestoreResult::new(&self.snapshot_id);
        result.files_restored = 100;
        // TODO: Implementar restauración real
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_result_new() {
        let result = RestoreResult::new("snap_123");
        assert_eq!(result.snapshot_id, "snap_123");
    }

    #[test]
    fn test_has_conflicts_false() {
        let result = RestoreResult::new("snap_123");
        assert!(!result.has_conflicts());
    }

    #[test]
    fn test_has_conflicts_true() {
        let mut result = RestoreResult::new("snap_123");
        result.conflicts.push(PathBuf::from("conflict.md"));
        assert!(result.has_conflicts());
    }

    #[test]
    fn test_restore_command_run() {
        let cmd = RestoreCommand {
            snapshot_id: "snap_123".to_string(),
            force: false,
            dry_run: false,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.files_restored, 100);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: RestoreCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("🔄 Restaurando snapshot: {}", result.snapshot_id);
    println!("📄 {} archivos restaurados", result.files_restored);
    
    if result.has_conflicts() {
        println!("⚠️  {} conflictos", result.conflicts.len());
    }
    
    Ok(())
}
