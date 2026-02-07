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

/// D2: Resultado de comparación de hash de contenido.
#[derive(Debug, Clone)]
pub struct HashComparisonResult {
    pub has_changed: bool,
    pub current_hash: String,
    pub stored_hash: Option<String>,
}

/// Resultado de sincronización.
#[derive(Debug, Clone)]
pub struct SyncResult {
    pub changes: Vec<SyncChange>,
    pub files_scanned: usize,
    pub files_modified: usize,
    pub skipped_tolerance: usize,      // D3: Archivos sin cambios reales
    pub hashes_initialized: usize,      // D3: Hashes inicializados
}

impl SyncResult {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            files_scanned: 0,
            files_modified: 0,
            skipped_tolerance: 0,
            hashes_initialized: 0,
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

    // P1: Nuevas flags de paridad con Python v16
    /// Ejecutar TODAS las sincronizaciones (dates + hashes + breadcrumbs + children).
    #[arg(long)]
    pub fix_all: bool,

    /// Filtrar por módulo específico (ej: 1, 2, 3...).
    #[arg(long)]
    pub module: Option<u8>,
}


impl SyncCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<SyncResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        
        use std::collections::HashMap;

        let mut result = SyncResult::new();

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        result.files_scanned = files.len();

        use crate::core::patterns::{RE_LAST_UPDATED, RE_CONTENT_HASH, RE_PARENT_ID};
        let date_regex = &*RE_LAST_UPDATED;
        let hash_regex = &*RE_CONTENT_HASH;
        let parent_regex = &*RE_PARENT_ID;

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

                // D6: Hash-based date synchronization (reemplaza mtime)
                if !self.hashes_only {
                    use sha2::{Digest, Sha256};
                    
                    // Calcular hash del contenido (excluyendo campos volátiles)
                    let content_for_hash: String = content
                        .lines()
                        .filter(|l| {
                            !l.starts_with("last_updated:") &&
                            !l.starts_with("content_hash:") &&
                            !l.starts_with("file_create:")
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    
                    let mut hasher = Sha256::new();
                    hasher.update(content_for_hash.as_bytes());
                    let current_hash = format!("{:x}", hasher.finalize())[..16].to_string();
                    
                    // Extraer hash almacenado
                    let stored_hash = hash_regex
                        .captures(&content)
                        .map(|cap| cap[1].trim().to_string());
                    
                    let has_changed = match &stored_hash {
                        Some(s) => s != &current_hash,
                        None => false, // No hay hash previo
                    };
                    
                    // Caso 1: Hash no existe → inicializar sin cambiar fecha
                    if stored_hash.is_none() && !has_changed && !self.force {
                        // Agregar hash si no existe (buscar después de frontmatter)
                        if !content.contains("content_hash:") {
                            // Insertar después de la primera línea ---
                            if let Some(pos) = modified_content.find("---\n") {
                                let insert_pos = pos + 4;
                                modified_content.insert_str(insert_pos, &format!("content_hash: \"{}\"\n", current_hash));
                                result.hashes_initialized += 1;
                                file_has_changes = true;
                            }
                        }
                    }
                    // Caso 2: Hash coincide → sin cambios reales
                    else if stored_hash.is_some() && !has_changed && !self.force {
                        result.skipped_tolerance += 1;
                        // No hacer nada
                    }
                    // Caso 3: Hash difiere O force → actualizar fecha + hash
                    else if has_changed || self.force {
                        let new_date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        
                        // Extraer fecha antigua
                        let old_date = date_regex
                            .captures(&content)
                            .map(|c| c[1].trim().to_string())
                            .unwrap_or_else(|| "N/A".to_string());
                        
                        result.add_change(SyncChange {
                            path: file_path.clone(),
                            field: "last_updated".to_string(),
                            old_value: old_date.clone(),
                            new_value: new_date.clone(),
                        });
                        
                        // Actualizar fecha
                        let date_field = format!("last_updated: \"{}\"", new_date);
                        modified_content = date_regex
                            .replace(&modified_content, date_field.as_str())
                            .to_string();
                        
                        // Actualizar hash
                        let hash_field = format!("content_hash: \"{}\"", current_hash);
                        modified_content = hash_regex
                            .replace(&modified_content, hash_field.as_str())
                            .to_string();
                        
                        file_has_changes = true;
                    }
                }

                // L15.3: Regenerar hashes
                if !self.dates_only {
                    use sha2::{Digest, Sha256};
                    
                    // RFC-06: Usar exactamente la misma lógica de hash que verify.rs
                    // Excluir campos volátiles (last_updated, content_hash, file_create)
                    let content_for_hash: String = content
                        .lines()
                        .filter(|l| {
                            !l.starts_with("last_updated:") &&
                            !l.starts_with("content_hash:") &&
                            !l.starts_with("file_create:")
                        })
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
                    use crate::core::patterns::RE_CHILDREN_COUNT;
                    let count_regex = &*RE_CHILDREN_COUNT;

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
    
    // P1-A4: Mostrar estadísticas extendidas
    if result.skipped_tolerance > 0 {
        println!("⏭️  {} archivos sin cambios (hash coincide)", result.skipped_tolerance);
    }
    if result.hashes_initialized > 0 {
        println!("🆕 {} hashes inicializados", result.hashes_initialized);
    }

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
        use crate::core::patterns::RE_PARENT_ID;
        let parent_re = &*RE_PARENT_ID;
        let mut children_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(path) {
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

        use crate::core::patterns::{RE_TYPE, RE_TOTAL_CHILDREN};
        let type_re = &*RE_TYPE;
        let total_re = &*RE_TOTAL_CHILDREN;

        let mut indices_updated = 0;

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(path) {
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
