//! Comando fix - Corrección automática de anomalías estructurales.
//!
//! RFC-07: Corrige tablas de contenido (Nietos) y otras anomalías.

use crate::errors::{OcError, OcResult};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════════════════
// TIPOS Y ESTRUCTURAS
// ═══════════════════════════════════════════════════════════════════════════

/// Un cambio de corrección aplicado.
#[derive(Debug, Clone)]
pub struct FixChange {
    pub path: PathBuf,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

/// Resultado de corrección.
#[derive(Debug, Clone, Default)]
pub struct FixResult {
    pub files_scanned: usize,
    pub files_fixed: usize,
    pub rows_updated: usize,
    pub changes: Vec<FixChange>,
}

impl FixResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_change(&mut self, change: FixChange) {
        self.changes.push(change);
    }
}

/// Comando de corrección de anomalías.
#[derive(Parser, Debug, Clone)]
#[command(name = "fix", about = "Corregir anomalías estructurales automáticamente")]
pub struct FixCommand {
    /// Ruta objetivo (default: directorio de datos).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Corregir tablas de contenido (columna Nietos).
    #[arg(long)]
    pub tables: bool,

    /// FIX#1: Sincronizar fechas YAML con filesystem.
    #[arg(long, help = "Sincronizar campo last_updated con fecha del filesystem")]
    pub dates: bool,

    /// FIX#2: Recalcular hashes de contenido.
    #[arg(long, help = "Recalcular campo content_hash basado en el contenido actual")]
    pub hashes: bool,

    /// Modo dry-run: mostrar cambios sin aplicar.
    #[arg(long)]
    pub dry_run: bool,

    /// Verbose: mostrar detalles de cada corrección.
    #[arg(short, long)]
    pub verbose: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// IMPLEMENTACIÓN DE FIXCOMMAND
// ═══════════════════════════════════════════════════════════════════════════

impl FixCommand {
    pub fn run(&self, data_dir: &Path) -> OcResult<FixResult> {
        use crate::core::files::{get_all_md_files, ScanOptions};

        let mut result = FixResult::new();
        let target = self.path.as_ref().map(|p| p.as_path()).unwrap_or(data_dir);

        let options = ScanOptions::new();
        let files = get_all_md_files(target, &options)?;
        result.files_scanned = files.len();

        if self.tables {
            // Paso 1: Recolectar todos los IDs de archivos
            let all_ids = self.collect_all_ids(&files);

            // Paso 2: Para cada archivo con tabla de contenido, corregir Nietos
            for file_path in &files {
                match self.fix_nietos_in_file(file_path, &all_ids, self.dry_run) {
                    Ok(updates) if updates > 0 => {
                        result.files_fixed += 1;
                        result.rows_updated += updates;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        if self.verbose {
                            eprintln!("⚠ Error procesando {:?}: {}", file_path, e);
                        }
                    }
                }
            }
        }

        // FIX#1: Sincronizar fechas YAML con filesystem
        if self.dates {
            let (fixed, updated) = self.fix_dates(&files, self.dry_run, self.verbose)?;
            result.files_fixed += fixed;
            result.rows_updated += updated;
        }

        // FIX#2: Recalcular hashes de contenido
        if self.hashes {
            let (fixed, updated) = self.fix_hashes(&files, self.dry_run, self.verbose)?;
            result.files_fixed += fixed;
            result.rows_updated += updated;
        }

        Ok(result)
    }

    /// Recolecta todos los IDs de archivos (basado en nombre de archivo).
    fn collect_all_ids(&self, files: &[PathBuf]) -> Vec<String> {
        files
            .iter()
            .filter_map(|p| {
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| Self::extract_id(s))
            })
            .flatten()
            .collect()
    }

    /// Extrae el ID numérico del nombre del archivo (ej: "1.2.3 nombre" -> "1.2.3").
    fn extract_id(filename: &str) -> Option<String> {
        // Regex para extraer ID numérico al inicio
        let re = Regex::new(r"^(\d+(?:\.\d+)*)").ok()?;
        re.captures(filename).map(|cap| cap[1].to_string())
    }

    /// Cuenta descendientes de un ID dado.
    /// BUGFIX L013: Ahora usa regex para contar solo descendientes numéricos.
    /// Cuenta TODOS los descendientes (hijos, nietos, bisnietos) cuyo ID empiece con prefix.digit
    /// Excluye archivos padre como "1.1. identidad" porque después del punto NO hay dígito
    fn count_descendants(id: &str, all_ids: &[String]) -> usize {
        use regex::Regex;
        
        // Patrón: id seguido de punto y dígito (sin $ para incluir todos los descendientes)
        // "1.1" matchea "1.1.0", "1.1.1", "1.1.7.2", etc.
        // NO matchea "1.1. identidad" (después del punto hay espacio, no dígito)
        let pattern = format!(r"^{}\.(\d+)", regex::escape(id));
        let re = match Regex::new(&pattern) {
            Ok(r) => r,
            Err(_) => return 0,
        };
        
        all_ids.iter().filter(|other| re.is_match(other)).count()
    }

    /// Corrige los valores de Nietos en tablas de contenido de un archivo.
    fn fix_nietos_in_file(
        &self,
        path: &PathBuf,
        all_ids: &[String],
        dry_run: bool,
    ) -> OcResult<usize> {
        let content = fs::read_to_string(path)
            .map_err(|_| OcError::FileNotFound(path.clone()))?;

        // Buscar tabla de contenido (patrón: | ID | ... | Nietos | ...)
        let table_regex = Regex::new(r"^\|[^\|]+\|[^\|]+\|[^\|]+\|[^\|]*Nietos[^\|]*\|")
            .map_err(|e| OcError::Custom(format!("Regex error: {}", e)))?;

        if !table_regex.is_match(&content) {
            return Ok(0); // No hay tabla de Nietos
        }

        // Parsear y corregir tabla
        let (new_content, updates) = self.parse_and_fix_table(&content, all_ids)?;

        if updates > 0 && !dry_run {
            fs::write(path, new_content)
                .map_err(|e| OcError::FileWrite { path: path.clone(), source: e })?;
        }

        Ok(updates)
    }

    /// Parsea y corrige una tabla de contenido.
    fn parse_and_fix_table(&self, content: &str, all_ids: &[String]) -> OcResult<(String, usize)> {
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        let mut updates = 0;
        let mut in_table = false;
        let mut nietos_col = None;
        let mut id_col = None;

        for (i, line) in lines.clone().iter().enumerate() {
            // Detectar inicio de tabla
            if line.starts_with('|') && line.ends_with('|') {
                if !in_table {
                    // Primera fila = header
                    in_table = true;
                    let cols: Vec<&str> = line.split('|').collect();

                    // Encontrar columnas ID y Nietos
                    for (j, col) in cols.iter().enumerate() {
                        let col_lower = col.trim().to_lowercase();
                        if col_lower == "id" || col_lower.starts_with("[[") {
                            id_col = Some(j);
                        }
                        if col_lower == "nietos" {
                            nietos_col = Some(j);
                        }
                    }
                    continue;
                }

                // Fila separadora (---)
                if line.contains("---") {
                    continue;
                }

                // Fila de datos
                if let (Some(id_idx), Some(nietos_idx)) = (id_col, nietos_col) {
                    let cols: Vec<&str> = line.split('|').collect();

                    if cols.len() > id_idx && cols.len() > nietos_idx {
                        // Extraer ID del wikilink
                        let id_cell = cols[id_idx].trim();
                        if let Some(id) = Self::extract_id_from_wikilink(id_cell) {
                            let actual_nietos = Self::count_descendants(&id, all_ids);
                            let declared_nietos: usize =
                                cols[nietos_idx].trim().parse().unwrap_or(0);

                            if declared_nietos != actual_nietos {
                                // Actualizar fila
                                let mut new_cols: Vec<String> = cols.iter().map(|c| c.to_string()).collect();
                                new_cols[nietos_idx] = format!(" {} ", actual_nietos);

                                let new_line = new_cols.join("|");
                                lines[i] = new_line;
                                updates += 1;

                                if self.verbose {
                                    println!(
                                        "  📝 {}: Nietos {} → {}",
                                        id, declared_nietos, actual_nietos
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                in_table = false;
                id_col = None;
                nietos_col = None;
            }
        }

        Ok((lines.join("\n"), updates))
    }

    /// Extrae ID de un wikilink (ej: "[[1.2.3 nombre]]" -> "1.2.3").
    fn extract_id_from_wikilink(cell: &str) -> Option<String> {
        let clean = cell.trim().trim_start_matches("[[").trim_end_matches("]]");
        Self::extract_id(clean)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FIX#1: SINCRONIZAR FECHAS YAML CON FILESYSTEM
    // ═══════════════════════════════════════════════════════════════════════

    /// FIX#1: Sincroniza el campo last_updated en YAML con la fecha del filesystem.
    fn fix_dates(
        &self,
        files: &[PathBuf],
        dry_run: bool,
        verbose: bool,
    ) -> OcResult<(usize, usize)> {
        use chrono::{Local, TimeZone};
        use std::time::UNIX_EPOCH;

        let mut files_fixed = 0;
        let mut fields_updated = 0;

        for path in files {
            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let re = match Regex::new(r#"last_updated:\s*\"?([^\"\n]+)\"?"#) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if let Some(cap) = re.captures(&content) {
                let old_date = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                
                let metadata = match fs::metadata(path) {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                
                let mtime = match metadata.modified() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                
                let duration = match mtime.duration_since(UNIX_EPOCH) {
                    Ok(d) => d,
                    Err(_) => continue,
                };
                
                let fs_secs = duration.as_secs() as i64;
                let dt = match Local.timestamp_opt(fs_secs, 0).single() {
                    Some(d) => d,
                    None => continue,
                };
                
                let new_date = dt.format("%Y-%m-%d %H:%M").to_string();
                
                // Parsear fecha YAML para comparar en segundos
                let yaml_secs = Self::parse_date_to_secs(old_date).unwrap_or(0) as i64;
                let diff_secs = (fs_secs - yaml_secs).abs();
                let diff_hours = diff_secs / 3600;
                
                // Solo corregir si la diferencia es >24 horas
                if diff_hours >= 24 {
                    let new_content = content.replace(
                        &cap[0],
                        &format!("last_updated: \"{}\"", new_date)
                    );
                    
                    if !dry_run {
                        fs::write(path, &new_content)?;
                    }
                    
                    files_fixed += 1;
                    fields_updated += 1;
                    
                    if verbose {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                        let mode = if dry_run { "[DRY-RUN] " } else { "" };
                        println!("📅 {}{}: {} → {} ({}h drift)", mode, name, old_date, new_date, diff_hours);
                    }
                }
            }
        }

        Ok((files_fixed, fields_updated))
    }

    /// Parsea fecha YAML a segundos desde UNIX_EPOCH.
    fn parse_date_to_secs(date_str: &str) -> Option<u64> {
        use chrono::{Local, NaiveDateTime, NaiveDate, TimeZone};
        
        let cleaned = date_str.trim().trim_matches('"');
        
        // Formato: "YYYY-MM-DD HH:MM:SS"
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%d %H:%M:%S") {
            if let Some(local_dt) = Local.from_local_datetime(&naive).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        // Formato: "YYYY-MM-DD HH:MM"
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%d %H:%M") {
            if let Some(local_dt) = Local.from_local_datetime(&naive).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        // Formato: "YYYY-MM-DD" (assume midnight)
        if let Ok(naive) = NaiveDate::parse_from_str(cleaned, "%Y-%m-%d") {
            let naive_dt = naive.and_hms_opt(0, 0, 0)?;
            if let Some(local_dt) = Local.from_local_datetime(&naive_dt).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        None
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FIX#2: RECALCULAR HASHES DE CONTENIDO
    // ═══════════════════════════════════════════════════════════════════════

    /// FIX#2: Recalcula el campo content_hash basado en el contenido actual.
    fn fix_hashes(
        &self,
        files: &[PathBuf],
        dry_run: bool,
        verbose: bool,
    ) -> OcResult<(usize, usize)> {
        use sha2::{Digest, Sha256};

        let mut files_fixed = 0;
        let mut fields_updated = 0;

        for path in files {
            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let re = match Regex::new(r#"content_hash:\s*\"?([^\"\n]+)\"?"#) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if let Some(cap) = re.captures(&content) {
                let old_hash = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                
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
                let new_hash = format!("{:x}", hasher.finalize());
                let new_hash = &new_hash[..16];
                
                if old_hash.trim() != new_hash {
                    let new_content = content.replace(
                        &cap[0],
                        &format!("content_hash: \"{}\"", new_hash)
                    );
                    
                    if !dry_run {
                        fs::write(path, &new_content)?;
                    }
                    
                    files_fixed += 1;
                    fields_updated += 1;
                    
                    if verbose {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                        let mode = if dry_run { "[DRY-RUN] " } else { "" };
                        let old_short = &old_hash[..8.min(old_hash.len())];
                        println!("🔐 {}{}: {} → {}", mode, name, old_short, new_hash);
                    }
                }
            }
        }

        Ok((files_fixed, fields_updated))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIÓN RUN PARA CLI
// ═══════════════════════════════════════════════════════════════════════════

/// Función run para CLI.
pub fn run(cmd: FixCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let data_dir = PathBuf::from(&cli.data_dir);

    println!("🔧 Iniciando corrección...");

    let result = cmd.run(&data_dir)?;

    // Imprimir resumen
    println!();
    if cmd.dry_run {
        println!(
            "ℹ️  [DRY-RUN] Se corregirían {} archivos ({} filas)",
            result.files_fixed,
            result.rows_updated
        );
    } else {
        println!(
            "✅ {} archivos corregidos ({} filas actualizadas)",
            result.files_fixed,
            result.rows_updated
        );
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_id() {
        assert_eq!(FixCommand::extract_id("1.2.3 nombre"), Some("1.2.3".to_string()));
        assert_eq!(FixCommand::extract_id("1.2.3.4.5 largo"), Some("1.2.3.4.5".to_string()));
        assert_eq!(FixCommand::extract_id("1 simple"), Some("1".to_string()));
        assert_eq!(FixCommand::extract_id("sin_id"), None);
    }

    #[test]
    fn test_extract_id_from_wikilink() {
        assert_eq!(
            FixCommand::extract_id_from_wikilink("[[1.2.3 nombre]]"),
            Some("1.2.3".to_string())
        );
        assert_eq!(
            FixCommand::extract_id_from_wikilink("  [[1.0 contexto]]  "),
            Some("1.0".to_string())
        );
    }

    #[test]
    fn test_count_descendants() {
        let all_ids = vec![
            "1".to_string(),
            "1.1".to_string(),
            "1.2".to_string(),
            "1.2.1".to_string(),
            "1.2.2".to_string(),
            "2".to_string(),
        ];

        // BUGFIX L013: Ahora cuenta TODOS los descendientes numéricos recursivamente
        // "1" tiene 4 descendientes: 1.1, 1.2, 1.2.1, 1.2.2
        // NO cuenta archivos padre como "1. algo" (espacio después del punto)
        assert_eq!(FixCommand::count_descendants("1", &all_ids), 4); // 1.1, 1.2, 1.2.1, 1.2.2
        assert_eq!(FixCommand::count_descendants("1.2", &all_ids), 2); // 1.2.1, 1.2.2
        assert_eq!(FixCommand::count_descendants("2", &all_ids), 0);
        
        // Test adicional: No contar archivo padre (espacio después del punto)
        let ids_with_parent = vec![
            "1.1".to_string(),       // Padre (no debe contarse porque no empieza con "1.1.")
            "1.1.0".to_string(),     // Descendiente
            "1.1.1".to_string(),     // Descendiente
            "1.1.1.2".to_string(),   // Descendiente anidado
        ];
        assert_eq!(FixCommand::count_descendants("1.1", &ids_with_parent), 3); // 1.1.0, 1.1.1, 1.1.1.2
    }
}
