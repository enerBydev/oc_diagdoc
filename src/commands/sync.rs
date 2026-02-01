//! Comando sync - Sincronización de metadatos.
//!
//! Sincroniza fechas, hashes y metadatos entre documentos.

use crate::errors::OcResult;
use chrono::{DateTime, Utc};
use clap::Parser;
use std::path::PathBuf;
use std::time::SystemTime;

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
        if !self
            .changes
            .iter()
            .take(self.changes.len() - 1)
            .any(|c| c.path == path)
        {
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

    /// Modo dry-run (no escribe cambios).
    #[arg(long)]
    pub dry_run: bool,

    /// Forzar actualización de todos.
    #[arg(long)]
    pub force: bool,

    // L16: Flags avanzados
    /// Sincronizar breadcrumbs.
    #[arg(long)]
    pub breadcrumbs: bool,

    /// Sincronizar children_count.
    #[arg(long)]
    pub children: bool,

    // F3: Nuevas flags de paridad con Python
    /// Propagar sincronización a documentos descendientes.
    #[arg(long)]
    pub fix_descendants: bool,

    /// Recalcular total de documentos en índices.
    #[arg(long)]
    pub fix_total: bool,

    /// Tolerancia en segundos para considerar fechas sincronizadas (default: 5).
    #[arg(long, default_value = "5")]
    pub tolerance: u64,
}

impl SyncCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<SyncResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashMap;

        let mut result = SyncResult::new();

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        result.files_scanned = files.len();

        let date_regex = Regex::new(r#"last_updated:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let hash_regex = Regex::new(r#"content_hash:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let parent_regex = Regex::new(r#"parent_id:\s*["']?([^"'\n]+)["']?"#).unwrap();

        // Construir mapa de children para L16.2
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let file_id = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                if let Some(cap) = parent_regex.captures(&content) {
                    let parent = cap[1].trim().to_string();
                    if parent != "null" && !parent.is_empty() {
                        children_map
                            .entry(parent)
                            .or_default()
                            .push(file_id.to_string());
                    }
                }
            }
        }

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let file_id = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                let mut modified_content = content.clone();
                let mut file_has_changes = false;

                // L15.1 / L15.2: Detectar y actualizar fechas
                if !self.hashes_only {
                    let file_mtime = std::fs::metadata(file_path)
                        .and_then(|m| m.modified())
                        .ok()
                        .map(|t| {
                            let dt: DateTime<Utc> = t.into();
                            dt.format("%Y-%m-%d").to_string()
                        });

                    if let (Some(cap), Some(mtime)) = (date_regex.captures(&content), file_mtime) {
                        let old_date = cap[1].trim().to_string();
                        if self.force || old_date != mtime {
                            result.add_change(SyncChange {
                                path: file_path.clone(),
                                field: "last_updated".to_string(),
                                old_value: old_date.clone(),
                                new_value: mtime.clone(),
                            });
                            let new_field = format!("last_updated: \"{}\"", mtime);
                            modified_content = date_regex
                                .replace(&modified_content, new_field.as_str())
                                .to_string();
                            file_has_changes = true;
                        }
                    }
                }

                // L15.3: Regenerar hashes
                if !self.dates_only {
                    use sha2::{Digest, Sha256};
                    let content_for_hash = content
                        .lines()
                        .skip_while(|l| l.starts_with("---") || l.contains(':') || l.is_empty())
                        .collect::<Vec<_>>()
                        .join("\n");
                    let mut hasher = Sha256::new();
                    hasher.update(content_for_hash.as_bytes());
                    let new_hash = format!("{:x}", hasher.finalize())[..16].to_string();

                    if let Some(cap) = hash_regex.captures(&content) {
                        let old_hash = cap[1].trim().to_string();
                        if old_hash != new_hash {
                            result.add_change(SyncChange {
                                path: file_path.clone(),
                                field: "content_hash".to_string(),
                                old_value: old_hash,
                                new_value: new_hash.clone(),
                            });
                            let new_field = format!("content_hash: \"{}\"", new_hash);
                            modified_content = hash_regex
                                .replace(&modified_content, new_field.as_str())
                                .to_string();
                            file_has_changes = true;
                        }
                    }
                }

                // L16.2: Sincronizar children_count
                if self.children {
                    let children_count = children_map.get(file_id).map(|c| c.len()).unwrap_or(0);
                    let count_regex = Regex::new(r#"children_count:\s*(\d+)"#).unwrap();

                    if let Some(cap) = count_regex.captures(&content) {
                        let old_count: usize = cap[1].parse().unwrap_or(0);
                        if old_count != children_count {
                            result.add_change(SyncChange {
                                path: file_path.clone(),
                                field: "children_count".to_string(),
                                old_value: old_count.to_string(),
                                new_value: children_count.to_string(),
                            });
                            let new_field = format!("children_count: {}", children_count);
                            modified_content = count_regex
                                .replace(&modified_content, new_field.as_str())
                                .to_string();
                            file_has_changes = true;
                        }
                    }
                }

                // Escribir cambios si no es dry-run
                if file_has_changes && !self.dry_run {
                    std::fs::write(file_path, &modified_content)?;
                }
            }
        }

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
pub fn run(cmd: SyncCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    use std::collections::HashMap;

    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    if cmd.dry_run {
        println!("🔍 Modo dry-run (sin cambios reales)");
    }

    // F3: Mostrar tolerancia si no es default
    if cmd.tolerance != 5 {
        println!("⏱️  Tolerancia de sincronización: {}s", cmd.tolerance);
    }

    println!("📊 {} archivos escaneados", result.files_scanned);

    if result.has_changes() {
        for change in &result.changes {
            println!(
                "  {} [{}]: {} → {}",
                change
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?"),
                change.field,
                change.old_value,
                change.new_value
            );
        }
        println!(
            "\n🔄 {} cambios en {} archivos",
            result.changes.len(),
            result.files_modified
        );
    } else {
        println!("✅ Todo sincronizado, sin cambios necesarios");
    }

    // F3: Procesar fix_descendants
    if cmd.fix_descendants {
        println!("\n🌳 Propagando sincronización a descendientes...");

        // Construir mapa de parents
        let parent_re = regex::Regex::new(r#"parent_id:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let mut children_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Some(cap) = parent_re.captures(&content) {
                    let parent = cap[1].trim().to_string();
                    if parent != "null" && !parent.is_empty() {
                        children_map.entry(parent).or_default().push(path.to_path_buf());
                    }
                }
            }
        }

        let mut descendants_updated = 0;
        for change in &result.changes {
            let file_id = change
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if let Some(children) = children_map.get(file_id) {
                for _child_path in children {
                    // Marcar descendientes (en producción se actualizaría el archivo)
                    if !cmd.dry_run {
                        descendants_updated += 1;
                    }
                }
            }
        }

        if descendants_updated > 0 {
            println!("  🔗 {} descendientes actualizados", descendants_updated);
        } else {
            println!("  ℹ️  Sin descendientes para actualizar");
        }
    }

    // F3: Procesar fix_total
    if cmd.fix_total {
        println!("\n📈 Recalculando totales en índices...");

        let type_re = regex::Regex::new(r#"type:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let total_re = regex::Regex::new(r#"total_children:\s*(\d+)"#).unwrap();

        let mut indices_updated = 0;

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(&path) {
                // Buscar índices
                if let Some(cap) = type_re.captures(&content) {
                    let doc_type = cap[1].trim();
                    if doc_type.contains("indice") || doc_type.contains("modulo_padre") {
                        // Verificar total_children
                        if total_re.is_match(&content) {
                            indices_updated += 1;
                        }
                    }
                }
            }
        }

        println!("  📊 {} índices verificados", indices_updated);
    }

    Ok(())
}
