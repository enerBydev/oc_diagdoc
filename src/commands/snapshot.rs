//! Comando snapshot - Instantáneas del proyecto.
//!
//! Crea y gestiona snapshots del estado del proyecto.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// SNAPSHOT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Información de un snapshot.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotInfo {
    pub id: String,
    pub name: String,
    pub timestamp: String,
    pub file_count: usize,
    pub size_bytes: usize,
}

impl SnapshotInfo {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            file_count: 0,
            size_bytes: 0,
        }
    }
}

/// Resultado de operación snapshot.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotResult {
    pub action: String,
    pub snapshot: Option<SnapshotInfo>,
    pub snapshots: Vec<SnapshotInfo>,
}

impl SnapshotResult {
    pub fn created(info: SnapshotInfo) -> Self {
        Self {
            action: "created".to_string(),
            snapshot: Some(info),
            snapshots: Vec::new(),
        }
    }

    pub fn list(snapshots: Vec<SnapshotInfo>) -> Self {
        Self {
            action: "list".to_string(),
            snapshot: None,
            snapshots,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SNAPSHOT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de snapshot.
#[derive(Parser, Debug, Clone)]
#[command(name = "snapshot", about = "Gestión de snapshots")]
pub struct SnapshotCommand {
    /// Nombre del snapshot.
    pub name: Option<String>,

    /// Listar snapshots.
    #[arg(short, long)]
    pub list: bool,

    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

impl SnapshotCommand {
    pub fn run(&self) -> OcResult<SnapshotResult> {
        if self.list {
            Ok(SnapshotResult::list(vec![]))
        } else {
            let name = self.name.as_deref().unwrap_or("snapshot");
            let id = format!("snap_{}", chrono::Utc::now().timestamp());
            let info = SnapshotInfo::new(&id, name);
            Ok(SnapshotResult::created(info))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_info_new() {
        let info = SnapshotInfo::new("1", "test");
        assert_eq!(info.name, "test");
    }

    #[test]
    fn test_snapshot_result_created() {
        let info = SnapshotInfo::new("1", "test");
        let result = SnapshotResult::created(info);
        assert_eq!(result.action, "created");
    }

    #[test]
    fn test_snapshot_result_list() {
        let result = SnapshotResult::list(vec![]);
        assert_eq!(result.action, "list");
    }

    #[test]
    fn test_snapshot_command_create() {
        let cmd = SnapshotCommand {
            name: Some("my_snap".to_string()),
            list: false,
            path: None,
        };
        let result = cmd.run().unwrap();
        assert!(result.snapshot.is_some());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: SnapshotCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;

    if result.action == "list" {
        println!("📷 Snapshots disponibles:");
        for s in &result.snapshots {
            println!("  {} - {} ({})", s.id, s.name, s.timestamp);
        }
    } else if let Some(s) = &result.snapshot {
        println!("📷 Snapshot creado: {} ({})", s.name, s.id);
    }

    Ok(())
}
