//! Comando archive - Archivado de documentos.
//!
//! Archiva documentos obsoletos o completados.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// ARCHIVE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de archivado.
#[derive(Debug, Clone, Serialize)]
pub struct ArchiveResult {
    pub archive_path: PathBuf,
    pub files_archived: usize,
    pub total_bytes: usize,
}

impl ArchiveResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            archive_path: path,
            files_archived: 0,
            total_bytes: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ARCHIVE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de archivado.
#[derive(Parser, Debug, Clone)]
#[command(name = "archive", about = "Archivar documentos")]
pub struct ArchiveCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Archivo de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Archivar documentos con status específico.
    #[arg(long)]
    pub status: Option<String>,

    /// Comprimir archivo.
    #[arg(long)]
    pub compress: bool,

    // L27-L28: Flags avanzados
    /// ID del documento a archivar.
    #[arg(long)]
    pub doc_id: Option<String>,

    /// Modo dry-run (no mover archivos).
    #[arg(long)]
    pub dry_run: bool,

    /// Actualizar referencias a docs archivados.
    #[arg(long)]
    pub update_refs: bool,
}

impl ArchiveCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<ArchiveResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        

        let archive_dir = data_dir.join("_archived");
        let mut result = ArchiveResult::new(archive_dir.clone());

        // Crear directorio _archived si no existe
        if !self.dry_run && !archive_dir.exists() {
            std::fs::create_dir_all(&archive_dir)?;
        }

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        use crate::core::patterns::{RE_STATUS, RE_DOCUMENT_ID};
        let status_regex = &*RE_STATUS;
        let id_regex = &*RE_DOCUMENT_ID;

        let filter_status = self
            .status
            .as_deref()
            .unwrap_or("deprecated,obsolete,archived");
        let statuses: Vec<_> = filter_status.split(',').map(|s| s.trim()).collect();

        let mut archived_ids: Vec<String> = Vec::new();

        for file_path in &files {
            if file_path.starts_with(&archive_dir) {
                continue; // Skip already archived
            }

            if let Ok(content) = read_file_content(file_path) {
                let mut should_archive = false;

                // L27.1: Archivar por status
                if let Some(cap) = status_regex.captures(&content) {
                    let file_status = cap[1].trim().to_lowercase();
                    if statuses
                        .iter()
                        .any(|s| file_status.contains(&s.to_lowercase()))
                    {
                        should_archive = true;
                    }
                }

                // L27.1: Archivar por doc_id específico
                if let Some(ref target_id) = self.doc_id {
                    if let Some(cap) = id_regex.captures(&content) {
                        if cap[1].trim() == target_id {
                            should_archive = true;
                            archived_ids.push(target_id.clone());
                        }
                    }
                }

                if should_archive {
                    // L27.2: Mover a _archived/
                    if let Some(file_name) = file_path.file_name() {
                        let dest = archive_dir.join(file_name);

                        if self.dry_run {
                            eprintln!(
                                "  📦 [DRY] Archivaría: {} → {}",
                                file_path.display(),
                                dest.display()
                            );
                        } else {
                            std::fs::copy(file_path, &dest)?;
                            std::fs::remove_file(file_path)?;
                            eprintln!(
                                "  📦 Archivado: {} → {}",
                                file_path.display(),
                                dest.display()
                            );
                        }

                        if let Ok(meta) = std::fs::metadata(&dest) {
                            result.total_bytes += meta.len() as usize;
                        }
                        result.files_archived += 1;

                        // Capturar ID para actualizar referencias
                        if let Some(cap) = id_regex.captures(&content) {
                            archived_ids.push(cap[1].trim().to_string());
                        }
                    }
                }
            }
        }

        // L28.1: Actualizar referencias
        if self.update_refs && !archived_ids.is_empty() {
            self.update_references(data_dir, &archived_ids)?;
        }

        Ok(result)
    }

    /// L28.1: Actualizar referencias a documentos archivados.
    fn update_references(
        &self,
        data_dir: &std::path::Path,
        archived_ids: &[String],
    ) -> OcResult<()> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let mut new_content = content.clone();
                let mut changed = false;

                for id in archived_ids {
                    // Buscar referencias al documento archivado
                    let patterns = vec![
                        format!("[[{}]]", id),
                        format!("[{}]", id),
                        format!("parent_id: {}", id),
                    ];

                    for pattern in patterns {
                        if new_content.contains(&pattern) {
                            let replacement = format!("{} (ARCHIVADO)", pattern);
                            new_content = new_content.replace(&pattern, &replacement);
                            changed = true;
                        }
                    }
                }

                if changed && !self.dry_run {
                    std::fs::write(file_path, &new_content)?;
                    eprintln!("  🔗 Referencias actualizadas: {}", file_path.display());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_result_new() {
        let result = ArchiveResult::new(PathBuf::from("out.tar.gz"));
        assert_eq!(result.files_archived, 0);
    }

    #[test]
    fn test_archive_command_fields() {
        let cmd = ArchiveCommand {
            path: None,
            output: Some(PathBuf::from("test.tar.gz")),
            status: None,
            compress: true,
            doc_id: None,
            dry_run: true,
            update_refs: false,
        };
        assert!(cmd.dry_run);
    }

    #[test]
    fn test_archive_with_status() {
        let cmd = ArchiveCommand {
            path: None,
            output: None,
            status: Some("deprecated".to_string()),
            compress: true,
            doc_id: None,
            dry_run: false,
            update_refs: true,
        };
        assert!(cmd.update_refs);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ArchiveCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    println!("📦 Archivando en: {}", result.archive_path.display());
    println!(
        "📄 {} archivos, {} bytes",
        result.files_archived, result.total_bytes
    );

    Ok(())
}
