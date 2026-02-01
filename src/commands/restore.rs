//! Comando restore - Restauración de snapshots.
//!
//! Restaura el proyecto a un estado previo.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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
    /// ID del snapshot o archivo a restaurar.
    pub snapshot_id: String,

    /// Forzar (sobrescribir conflictos).
    #[arg(short, long)]
    pub force: bool,

    /// Modo dry-run.
    #[arg(long)]
    pub dry_run: bool,

    // L29-L30: Flags avanzados
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Validar integridad antes de restaurar.
    #[arg(long)]
    pub validate: bool,

    /// Filtro de restauración (pattern).
    #[arg(long)]
    pub filter: Option<String>,
}

impl RestoreCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<RestoreResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;

        let mut result = RestoreResult::new(&self.snapshot_id);

        // L29.1: Restaurar desde _archived/
        let archive_dir = data_dir.join("_archived");
        if !archive_dir.exists() {
            return Err(crate::errors::OcError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Directorio _archived no existe",
            )));
        }

        let options = ScanOptions::new();
        let archived_files = get_all_md_files(&archive_dir, &options)?;

        let id_regex = Regex::new(r#"document_id:\s*["']?([^"'\n]+)["']?"#).unwrap();

        for file_path in &archived_files {
            if let Ok(content) = read_file_content(file_path) {
                let mut should_restore = false;

                // Restaurar por snapshot_id (doc_id)
                if let Some(cap) = id_regex.captures(&content) {
                    if cap[1].trim() == self.snapshot_id {
                        should_restore = true;
                    }
                }

                // Restaurar por filename
                if let Some(file_name) = file_path.file_name() {
                    if file_name.to_string_lossy().contains(&self.snapshot_id) {
                        should_restore = true;
                    }
                }

                // L30.1: Filtro parcial
                if let Some(ref filter) = self.filter {
                    let pattern = Regex::new(filter).ok();
                    if let Some(ref re) = pattern {
                        if !re.is_match(&content) {
                            should_restore = false;
                        }
                    }
                }

                if should_restore {
                    // L29.2: Validar integridad
                    if self.validate && !self.validate_file(file_path, &content) {
                        eprintln!("  ⚠️ Archivo corrupto: {}", file_path.display());
                        result.files_skipped += 1;
                        continue;
                    }

                    if let Some(file_name) = file_path.file_name() {
                        let dest = data_dir.join("docs").join(file_name);

                        // Detectar conflictos
                        if dest.exists() && !self.force {
                            result.conflicts.push(dest.clone());
                            if !self.dry_run {
                                eprintln!("  ⚠️ Conflicto (use --force): {}", dest.display());
                            }
                            continue;
                        }

                        if self.dry_run {
                            eprintln!(
                                "  🔄 [DRY] Restauraría: {} → {}",
                                file_path.display(),
                                dest.display()
                            );
                        } else {
                            std::fs::create_dir_all(dest.parent().unwrap_or(data_dir))?;
                            std::fs::copy(file_path, &dest)?;
                            std::fs::remove_file(file_path)?;
                            eprintln!(
                                "  🔄 Restaurado: {} → {}",
                                file_path.display(),
                                dest.display()
                            );
                        }

                        result.files_restored += 1;
                    }
                }
            }
        }

        Ok(result)
    }

    /// L29.2: Validar integridad del archivo.
    fn validate_file(&self, _file_path: &PathBuf, content: &str) -> bool {
        // Validación básica: debe tener frontmatter
        let has_frontmatter = content.starts_with("---") && content.matches("---").count() >= 2;

        // Debe tener document_id
        let has_doc_id = content.contains("document_id:");

        // Debe tener título
        let has_title = content.contains("title:");

        has_frontmatter && has_doc_id && has_title
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
    fn test_restore_command_fields() {
        let cmd = RestoreCommand {
            snapshot_id: "snap_123".to_string(),
            force: false,
            dry_run: true,
            path: None,
            validate: true,
            filter: Some("modulo_1".to_string()),
        };
        assert!(cmd.validate);
        assert!(cmd.dry_run);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: RestoreCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    println!("🔄 Restaurando snapshot: {}", result.snapshot_id);
    println!("📄 {} archivos restaurados", result.files_restored);

    if result.files_skipped > 0 {
        println!("⏭️  {} archivos omitidos", result.files_skipped);
    }

    if result.has_conflicts() {
        println!("⚠️  {} conflictos", result.conflicts.len());
    }

    Ok(())
}
