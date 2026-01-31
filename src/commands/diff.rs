//! Comando diff - Comparación de estados.
//!
//! Compara estados del proyecto entre commits o snapshots.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// DIFF TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de cambio.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}

/// Un cambio detectado.
#[derive(Debug, Clone, Serialize)]
pub struct DiffChange {
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

/// Resultado del diff.
#[derive(Debug, Clone, Serialize)]
pub struct DiffResult {
    pub from_ref: String,
    pub to_ref: String,
    pub changes: Vec<DiffChange>,
}

impl DiffResult {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_ref: from.to_string(),
            to_ref: to.to_string(),
            changes: Vec::new(),
        }
    }
    
    pub fn added_count(&self) -> usize {
        self.changes.iter().filter(|c| c.change_type == ChangeType::Added).count()
    }
    
    pub fn modified_count(&self) -> usize {
        self.changes.iter().filter(|c| c.change_type == ChangeType::Modified).count()
    }
    
    pub fn deleted_count(&self) -> usize {
        self.changes.iter().filter(|c| c.change_type == ChangeType::Deleted).count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DIFF COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de diff.
#[derive(Parser, Debug, Clone)]
#[command(name = "diff", about = "Comparar estados")]
pub struct DiffCommand {
    /// Referencia inicial.
    #[arg(default_value = "HEAD~1")]
    pub from: String,
    
    /// Referencia final.
    #[arg(default_value = "HEAD")]
    pub to: String,
    
    /// Solo stats (sin detalles).
    #[arg(long)]
    pub stat: bool,
}

impl DiffCommand {
    pub fn run(&self) -> OcResult<DiffResult> {
        let result = DiffResult::new(&self.from, &self.to);
        // TODO: Implementar diff real
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_result_new() {
        let result = DiffResult::new("a", "b");
        assert_eq!(result.from_ref, "a");
    }

    #[test]
    fn test_diff_counts() {
        let mut result = DiffResult::new("a", "b");
        result.changes.push(DiffChange {
            path: PathBuf::from("a.md"),
            change_type: ChangeType::Added,
            old_value: None,
            new_value: Some("new".to_string()),
        });
        result.changes.push(DiffChange {
            path: PathBuf::from("b.md"),
            change_type: ChangeType::Modified,
            old_value: Some("old".to_string()),
            new_value: Some("new".to_string()),
        });
        
        assert_eq!(result.added_count(), 1);
        assert_eq!(result.modified_count(), 1);
    }

    #[test]
    fn test_change_type() {
        assert_eq!(ChangeType::Added, ChangeType::Added);
        assert_ne!(ChangeType::Added, ChangeType::Deleted);
    }

    #[test]
    fn test_diff_command_run() {
        let cmd = DiffCommand {
            from: "v1".to_string(),
            to: "v2".to_string(),
            stat: false,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.from_ref, "v1");
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: DiffCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    println!("📊 Diff: {} → {}", result.from_ref, result.to_ref);
    println!("  ➕ {} añadidos", result.added_count());
    println!("  ✏️  {} modificados", result.modified_count());
    println!("  ➖ {} eliminados", result.deleted_count());
    
    Ok(())
}
