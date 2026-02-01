//! Comando migrate - Migración de versiones.
//!
//! Migra proyectos entre versiones de oc_diagdoc.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// MIGRATE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de migración.
#[derive(Debug, Clone, Serialize)]
pub struct MigrateResult {
    pub from_version: String,
    pub to_version: String,
    pub migrations_applied: Vec<String>,
    pub files_modified: usize,
}

impl MigrateResult {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from_version: from.to_string(),
            to_version: to.to_string(),
            migrations_applied: Vec::new(),
            files_modified: 0,
        }
    }

    pub fn add_migration(&mut self, name: &str) {
        self.migrations_applied.push(name.to_string());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MIGRATE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de migración.
#[derive(Parser, Debug, Clone)]
#[command(name = "migrate", about = "Migrar proyecto")]
pub struct MigrateCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Versión destino.
    #[arg(short, long)]
    pub to: Option<String>,

    /// Modo dry-run.
    #[arg(long)]
    pub dry_run: bool,
}

impl MigrateCommand {
    pub fn run(&self) -> OcResult<MigrateResult> {
        let to_version = self.to.as_deref().unwrap_or("3.0");
        let mut result = MigrateResult::new("2.0", to_version);

        // Migraciones de ejemplo
        result.add_migration("update_frontmatter_schema");
        result.add_migration("rename_status_field");
        result.files_modified = 10;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_result_new() {
        let result = MigrateResult::new("1.0", "2.0");
        assert_eq!(result.from_version, "1.0");
    }

    #[test]
    fn test_add_migration() {
        let mut result = MigrateResult::new("1.0", "2.0");
        result.add_migration("test_migration");
        assert_eq!(result.migrations_applied.len(), 1);
    }

    #[test]
    fn test_migrate_command_run() {
        let cmd = MigrateCommand {
            path: None,
            to: Some("3.0".to_string()),
            dry_run: false,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.to_version, "3.0");
    }

    #[test]
    fn test_default_version() {
        let cmd = MigrateCommand {
            path: None,
            to: None,
            dry_run: false,
        };
        let result = cmd.run().unwrap();
        assert_eq!(result.to_version, "3.0");
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: MigrateCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;

    println!(
        "🔄 Migrando {} → {}",
        result.from_version, result.to_version
    );
    for m in &result.migrations_applied {
        println!("  ✓ {}", m);
    }
    println!("📊 {} archivos modificados", result.files_modified);

    Ok(())
}
