//! Comando diff - Comparación de estados.
//!
//! Compara estados del proyecto entre commits o snapshots.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Added)
            .count()
    }

    pub fn modified_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Modified)
            .count()
    }

    pub fn deleted_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|c| c.change_type == ChangeType::Deleted)
            .count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DIFF COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de diff.
#[derive(Parser, Debug, Clone)]
#[command(name = "diff", about = "Comparar estados")]
pub struct DiffCommand {
    /// Referencia inicial (directorio o snapshot).
    #[arg(default_value = "HEAD~1")]
    pub from: String,

    /// Referencia final.
    #[arg(default_value = "HEAD")]
    pub to: String,

    /// Solo stats (sin detalles).
    #[arg(long)]
    pub stat: bool,

    // L21-L22: Flags avanzados
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Mostrar diff side-by-side.
    #[arg(long)]
    pub side_by_side: bool,

    /// Limitar líneas de contexto.
    #[arg(short = 'c', long, default_value = "3")]
    pub context: usize,
}

impl DiffCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<DiffResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use std::collections::HashSet;

        let mut result = DiffResult::new(&self.from, &self.to);

        // L21.1: Comparar dos directorios/snapshots
        let from_dir = if self.from.starts_with('/') || self.from.starts_with('.') {
            PathBuf::from(&self.from)
        } else {
            data_dir.join(&self.from)
        };

        let to_dir = if self.to.starts_with('/') || self.to.starts_with('.') {
            PathBuf::from(&self.to)
        } else {
            data_dir.to_path_buf()
        };

        // Obtener archivos de ambos directorios
        let options = ScanOptions::new();
        let from_files: HashSet<_> = get_all_md_files(&from_dir, &options)
            .unwrap_or_default()
            .iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .collect();

        let to_files: HashSet<_> = get_all_md_files(&to_dir, &options)
            .unwrap_or_default()
            .iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
            })
            .collect();

        // L21.2: Detectar añadidos
        for file in to_files.difference(&from_files) {
            result.changes.push(DiffChange {
                path: PathBuf::from(file),
                change_type: ChangeType::Added,
                old_value: None,
                new_value: Some("[nuevo archivo]".to_string()),
            });
        }

        // L21.2: Detectar eliminados
        for file in from_files.difference(&to_files) {
            result.changes.push(DiffChange {
                path: PathBuf::from(file),
                change_type: ChangeType::Deleted,
                old_value: Some("[archivo eliminado]".to_string()),
                new_value: None,
            });
        }

        // L21.2: Detectar modificados (archivos en ambos)
        for file in from_files.intersection(&to_files) {
            let from_path = from_dir.join(file);
            let to_path = to_dir.join(file);

            if let (Ok(from_content), Ok(to_content)) =
                (read_file_content(&from_path), read_file_content(&to_path))
            {
                if from_content != to_content {
                    let lines_from = from_content.lines().count();
                    let lines_to = to_content.lines().count();
                    let diff_summary = format!(
                        "+{} -{} líneas",
                        lines_to.saturating_sub(lines_from),
                        lines_from.saturating_sub(lines_to)
                    );

                    result.changes.push(DiffChange {
                        path: PathBuf::from(file),
                        change_type: ChangeType::Modified,
                        old_value: Some(format!("{} líneas", lines_from)),
                        new_value: Some(format!("{} líneas ({})", lines_to, diff_summary)),
                    });
                }
            }
        }

        Ok(result)
    }

    /// L22.1: Genera diff side-by-side para un archivo.
    pub fn render_side_by_side(from_content: &str, to_content: &str, width: usize) -> String {
        let half_width = width / 2 - 2;
        let from_lines: Vec<&str> = from_content.lines().collect();
        let to_lines: Vec<&str> = to_content.lines().collect();
        let max_lines = from_lines.len().max(to_lines.len());

        let mut output = String::new();
        output.push_str(&format!(
            "{:─^width$}\n",
            " DIFF SIDE-BY-SIDE ",
            width = width
        ));
        output.push_str(&format!(
            "{:^half$} │ {:^half$}\n",
            "FROM",
            "TO",
            half = half_width
        ));
        output.push_str(&format!(
            "{:─<half$}─┼─{:─<half$}\n",
            "",
            "",
            half = half_width
        ));

        for i in 0..max_lines {
            let left = from_lines.get(i).unwrap_or(&"");
            let right = to_lines.get(i).unwrap_or(&"");

            let left_truncated: String = left.chars().take(half_width).collect();
            let right_truncated: String = right.chars().take(half_width).collect();

            let marker = if left != right { "│*" } else { "│ " };
            output.push_str(&format!(
                "{:half$}{}{:half$}\n",
                left_truncated,
                marker,
                right_truncated,
                half = half_width
            ));
        }

        output
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
    fn test_diff_command_fields() {
        let cmd = DiffCommand {
            from: "v1".to_string(),
            to: "v2".to_string(),
            stat: false,
            path: None,
            side_by_side: true,
            context: 3,
        };
        assert!(cmd.side_by_side);
        assert_eq!(cmd.context, 3);
    }

    #[test]
    fn test_render_side_by_side() {
        let from = "line1\nline2\nline3";
        let to = "line1\nmodified\nline3";
        let output = DiffCommand::render_side_by_side(from, to, 60);
        assert!(output.contains("DIFF"));
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: DiffCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    println!("📊 Diff: {} → {}", result.from_ref, result.to_ref);
    println!("  ➕ {} añadidos", result.added_count());
    println!("  ✏️  {} modificados", result.modified_count());
    println!("  ➖ {} eliminados", result.deleted_count());

    if !cmd.stat && !result.changes.is_empty() {
        println!("\n📋 Cambios:");
        for change in &result.changes {
            let marker = match change.change_type {
                ChangeType::Added => "➕",
                ChangeType::Modified => "✏️",
                ChangeType::Deleted => "➖",
                ChangeType::Renamed => "📝",
            };
            println!("  {} {}", marker, change.path.display());
        }
    }

    Ok(())
}
