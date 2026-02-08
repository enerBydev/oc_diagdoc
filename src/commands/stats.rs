//! Comando stats - Dashboard de estadísticas.
//!
//! Muestra estadísticas completas del proyecto.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// STATS OUTPUT
// ═══════════════════════════════════════════════════════════════════════════

/// Estadísticas globales del proyecto.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectStats {
    /// Total de documentos.
    pub total_documents: usize,
    /// Documentos saludables.
    pub healthy_documents: usize,
    /// Total de palabras.
    pub total_words: usize,
    /// Total de enlaces.
    pub total_links: usize,
    /// Enlaces rotos.
    pub broken_links: usize,
    /// Módulos.
    pub modules_count: usize,
    /// Profundidad máxima.
    pub max_depth: usize,
}

impl ProjectStats {
    pub fn new() -> Self {
        Self {
            total_documents: 0,
            healthy_documents: 0,
            total_words: 0,
            total_links: 0,
            broken_links: 0,
            modules_count: 0,
            max_depth: 0,
        }
    }

    pub fn health_percent(&self) -> f64 {
        if self.total_documents == 0 {
            100.0
        } else {
            (self.healthy_documents as f64 / self.total_documents as f64) * 100.0
        }
    }

    pub fn avg_words_per_doc(&self) -> usize {
        if self.total_documents == 0 {
            0
        } else {
            self.total_words / self.total_documents
        }
    }
}

impl Default for ProjectStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Estadísticas por módulo.
#[derive(Debug, Clone, Serialize)]
pub struct ModuleStats {
    pub id: String,
    pub name: String,
    pub document_count: usize,
    pub word_count: usize,
    pub health_score: f64,
}

// ═══════════════════════════════════════════════════════════════════════════
// STATS COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de estadísticas.
#[derive(Parser, Debug, Clone)]
#[command(name = "stats", about = "Dashboard de estadísticas del proyecto")]
pub struct StatsCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Mostrar por módulo.
    #[arg(short, long)]
    pub by_module: bool,

    /// Output JSON.
    #[arg(long)]
    pub json: bool,

    /// Ordenar por campo.
    #[arg(long, default_value = "id")]
    pub sort: String,

    // F2: Nuevas flags de paridad con Python
    /// Agrupar estadísticas por status (activo, futuro, etc.).
    #[arg(long)]
    pub by_status: bool,

    /// Agrupar estadísticas por tipo de documento.
    #[arg(long)]
    pub by_type: bool,

    /// Mostrar los N archivos más recientes.
    #[arg(long)]
    pub recent: Option<usize>,

    /// Incluir información de tamaño en bytes.
    #[arg(long)]
    pub size: bool,

    // AN-03 FIX: Heatmap de actividad
    /// Mostrar mapa de calor de actividad por mes.
    #[arg(long)]
    pub heatmap: bool,

    /// P2-C3: Usar caché para estadísticas (sled).
    #[arg(long)]
    pub cache: bool,
}

impl StatsCommand {
    /// Ejecuta el comando.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<(ProjectStats, Vec<ModuleStats>)> {
        
        use std::collections::HashMap;
        use std::fs;

        use crate::core::patterns::RE_WIKI_LINK_WITH_ALIAS;
        let link_re = &*RE_WIKI_LINK_WITH_ALIAS;

        // Collect all .md files using WalkDir (RECURSIVE)
        use walkdir::WalkDir;
        let files: Vec<PathBuf> = WalkDir::new(data_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                if !path.is_file() { return false; }
                if path.extension().map(|ext| ext != "md").unwrap_or(true) { return false; }
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                !name.starts_with("TRAP_") && !name.starts_with("AUTOTEST_") && !name.starts_with("TEST_")
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        // Build file map for link checking
        let file_map: std::collections::HashSet<String> = files
            .iter()
            .filter_map(|p| {
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
            })
            .collect();

        let mut total_words = 0usize;
        let mut total_links = 0usize;
        let mut broken_links = 0usize;
        let mut healthy_documents = 0usize;
        let mut max_depth = 0usize;
        let mut module_map: HashMap<String, (usize, usize)> = HashMap::new(); // module_id -> (doc_count, word_count)

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                // Count words (skip YAML frontmatter)
                let body = if content.starts_with("---") {
                    if let Some(end) = content[3..].find("---") {
                        &content[3 + end + 3..]
                    } else {
                        &content
                    }
                } else {
                    &content
                };
                let words = body.split_whitespace().count();
                total_words += words;

                // Check if healthy (has YAML frontmatter)
                if content.starts_with("---") && content[3..].contains("---") {
                    healthy_documents += 1;
                }

                // Count links and check if broken
                // FIX BUG 1: Ignorar code blocks (sincronizado con links.rs)
                let mut in_code_block = false;
                for line in content.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                        in_code_block = !in_code_block;
                        continue;
                    }
                    if in_code_block {
                        continue;
                    }
                    
                    for cap in link_re.captures_iter(line) {
                        if let Some(m) = cap.get(1) {
                            let link_raw = m.as_str().trim().trim_end_matches('\\');
                            if !link_raw.is_empty() && !link_raw.starts_with("http") && !link_raw.starts_with('#') {
                                total_links += 1;

                                // FIX BUG 4: Normalizar escaped pipes
                                let link_clean = link_raw.replace("\\|", "|");
                                
                                // FIX BUG 3: Extraer nombre sin alias
                                let link_no_alias = link_clean.split('|').next().unwrap_or(&link_clean);
                                
                                // FIX BUG 2: Extraer nombre sin path
                                let link_no_path = link_no_alias.split('/').next_back().unwrap_or(link_no_alias);
                                
                                // Quitar anchor
                                let link_file = link_no_path.split('#').next().unwrap_or(link_no_path).trim();
                                
                                // FIX BUG 5: Usar fuzzy matching (sincronizado con links.rs)
                                let link_lower = link_file.to_lowercase();
                                let mut found = file_map.contains(&link_lower);
                                
                                if !found {
                                    // Fuzzy: match parcial (archivo termina con target o comienza con target)
                                    for file_name in &file_map {
                                        if file_name.ends_with(&link_lower) 
                                            || file_name.starts_with(&link_lower)
                                            || file_name.contains(&link_lower) {
                                            found = true;
                                            break;
                                        }
                                    }
                                }

                                if !found {
                                    broken_links += 1;
                                }
                            }
                        }
                    }
                }

                // Extract ID for depth and module stats
                if let Some(id) = Self::get_yaml_field(&content, "id") {
                    // Calculate depth from ID (e.g., "1.2.3" = depth 3)
                    let depth = id.matches('.').count() + 1;
                    if depth > max_depth {
                        max_depth = depth;
                    }

                    // Extract module (first number in ID)
                    let module_id = id.split('.').next().unwrap_or("0").to_string();
                    let entry = module_map.entry(module_id).or_insert((0, 0));
                    entry.0 += 1;
                    entry.1 += words;
                }
            }
        }

        let project_stats = ProjectStats {
            total_documents: files.len(),
            healthy_documents,
            total_words,
            total_links,
            broken_links,
            modules_count: module_map.len(),
            max_depth,
        };

        // Build module stats
        let mut module_stats: Vec<ModuleStats> = module_map
            .iter()
            .map(|(id, (doc_count, word_count))| {
                let health = if *doc_count > 0 { 100.0 } else { 0.0 };
                ModuleStats {
                    id: id.clone(),
                    name: format!("Módulo {}", id),
                    document_count: *doc_count,
                    word_count: *word_count,
                    health_score: health,
                }
            })
            .collect();

        // Sort by ID numerically
        module_stats.sort_by(|a, b| {
            a.id.parse::<u32>()
                .unwrap_or(0)
                .cmp(&b.id.parse::<u32>().unwrap_or(0))
        });

        Ok((project_stats, module_stats))
    }

    /// Helper to extract YAML field
    fn get_yaml_field(content: &str, field: &str) -> Option<String> {
        if !content.starts_with("---") {
            return None;
        }

        let end_idx = content[3..].find("---")?;
        let yaml_text = &content[3..3 + end_idx];

        for line in yaml_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("{}:", field)) {
                let value_part = trimmed.strip_prefix(&format!("{}:", field))?;
                let value = value_part.trim().trim_matches(|c| c == '"' || c == '\'');
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    /// Renderiza como tabla.
    pub fn render_table(stats: &ProjectStats) -> String {
        format!(
            "╔══════════════════════════════════════╗\n\
             ║           📊 PROJECT STATS           ║\n\
             ╠══════════════════════════════════════╣\n\
             ║ Documents:     {:>20} ║\n\
             ║ Healthy:       {:>20} ║\n\
             ║ Health:        {:>19.1}% ║\n\
             ║ Words:         {:>20} ║\n\
             ║ Avg Words/Doc: {:>20} ║\n\
             ║ Links:         {:>20} ║\n\
             ║ Broken Links:  {:>20} ║\n\
             ║ Modules:       {:>20} ║\n\
             ║ Max Depth:     {:>20} ║\n\
             ╚══════════════════════════════════════╝",
            stats.total_documents,
            stats.healthy_documents,
            stats.health_percent(),
            stats.total_words,
            stats.avg_words_per_doc(),
            stats.total_links,
            stats.broken_links,
            stats.modules_count,
            stats.max_depth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_stats_new() {
        let stats = ProjectStats::new();
        assert_eq!(stats.total_documents, 0);
    }

    #[test]
    fn test_health_percent() {
        let mut stats = ProjectStats::new();
        stats.total_documents = 100;
        stats.healthy_documents = 75;

        assert_eq!(stats.health_percent(), 75.0);
    }

    #[test]
    fn test_avg_words() {
        let mut stats = ProjectStats::new();
        stats.total_documents = 10;
        stats.total_words = 1000;

        assert_eq!(stats.avg_words_per_doc(), 100);
    }

    #[test]
    fn test_render_table() {
        let stats = ProjectStats::new();
        let output = StatsCommand::render_table(&stats);

        assert!(output.contains("PROJECT STATS"));
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: StatsCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    use std::collections::HashMap;

    // Priorizar cmd.path, sino usar cli.data_dir
    let data_dir = cmd
        .path
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from(&cli.data_dir));
    let (stats, module_stats) = cmd.run(&data_dir)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
        return Ok(());
    }

    // Render stats básicas
    println!("{}", StatsCommand::render_table(&stats));

    if cmd.by_module && !module_stats.is_empty() {
        println!("\n📦 Stats por módulo:");
        for ms in &module_stats {
            println!(
                "  {} ({}): {} docs, {} words",
                ms.name, ms.id, ms.document_count, ms.word_count
            );
        }
    }

    // F2: Nuevas funcionalidades
    // Recolectar datos adicionales si se requieren
    if cmd.by_status || cmd.by_type || cmd.recent.is_some() || cmd.size {
        let mut status_map: HashMap<String, usize> = HashMap::new();
        let mut type_map: HashMap<String, usize> = HashMap::new();
        let mut recent_files: Vec<(std::path::PathBuf, std::time::SystemTime, usize)> = Vec::new();
        let mut total_bytes: u64 = 0;

        // Use WalkDir for recursive scanning
        use walkdir::WalkDir;
        for entry in WalkDir::new(&data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("TRAP_") || name.starts_with("AUTOTEST_") || name.starts_with("TEST_") {
                continue;
            }

            // Size
            if cmd.size {
                if let Ok(meta) = std::fs::metadata(path) {
                    total_bytes += meta.len();
                }
            }

            // Recent files
            if cmd.recent.is_some() {
                if let Ok(meta) = std::fs::metadata(path) {
                    if let Ok(mtime) = meta.modified() {
                        let size = meta.len() as usize;
                        recent_files.push((path.to_path_buf(), mtime, size));
                    }
                }
            }

            // Status y Type
            if cmd.by_status || cmd.by_type {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if cmd.by_status {
                        let status = StatsCommand::get_yaml_field(&content, "status")
                            .unwrap_or_else(|| "sin_status".to_string());
                        *status_map.entry(status).or_insert(0) += 1;
                    }
                    if cmd.by_type {
                        let doc_type = StatsCommand::get_yaml_field(&content, "type")
                            .unwrap_or_else(|| "sin_tipo".to_string());
                        *type_map.entry(doc_type).or_insert(0) += 1;
                    }
                }
            }
        }

        // Mostrar por status
        if cmd.by_status && !status_map.is_empty() {
            println!("\n📊 Por Status:");
            let mut sorted: Vec<_> = status_map.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            for (status, count) in sorted {
                println!("  {:15} {:>5} docs", status, count);
            }
        }

        // Mostrar por tipo
        if cmd.by_type && !type_map.is_empty() {
            println!("\n📁 Por Tipo:");
            let mut sorted: Vec<_> = type_map.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            for (doc_type, count) in sorted.iter().take(15) {
                println!("  {:20} {:>5} docs", doc_type, count);
            }
            if sorted.len() > 15 {
                println!("  ... y {} tipos más", sorted.len() - 15);
            }
        }

        // Mostrar recientes
        if let Some(n) = cmd.recent {
            recent_files.sort_by(|a, b| b.1.cmp(&a.1));
            println!("\n⏰ {} Archivos más recientes:", n.min(recent_files.len()));
            for (path, mtime, size) in recent_files.iter().take(n) {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                let ago = mtime.elapsed().map(|d| d.as_secs()).unwrap_or(0);
                let ago_str = if ago < 60 {
                    format!("{}s", ago)
                } else if ago < 3600 {
                    format!("{}m", ago / 60)
                } else if ago < 86400 {
                    format!("{}h", ago / 3600)
                } else {
                    format!("{}d", ago / 86400)
                };
                println!("  {:40} {:>6} bytes  hace {}", name, size, ago_str);
            }
        }

        // Mostrar tamaño
        if cmd.size {
            println!("\n💾 Tamaño Total:");
            let kb = total_bytes as f64 / 1024.0;
            let mb = kb / 1024.0;
            if mb >= 1.0 {
                println!("  {:.2} MB ({} bytes)", mb, total_bytes);
            } else {
                println!("  {:.2} KB ({} bytes)", kb, total_bytes);
            }
            let avg = if stats.total_documents > 0 {
                total_bytes as f64 / stats.total_documents as f64
            } else {
                0.0
            };
            println!("  Promedio: {:.0} bytes/doc", avg);
        }

        // AN-03 FIX: Heatmap de actividad
        if cmd.heatmap {
            use std::collections::BTreeMap;
            let mut month_activity: BTreeMap<String, usize> = BTreeMap::new();
            
            for entry in WalkDir::new(&data_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if !path.is_file() || path.extension().map(|e| e != "md").unwrap_or(true) {
                    continue;
                }
                if let Ok(meta) = std::fs::metadata(path) {
                    if let Ok(modified) = meta.modified() {
                        let datetime: chrono::DateTime<chrono::Utc> = modified.into();
                        let key = datetime.format("%Y-%m").to_string();
                        *month_activity.entry(key).or_insert(0) += 1;
                    }
                }
            }
            
            println!("\n📊 Heatmap de Actividad (por mes):");
            let max = month_activity.values().max().copied().unwrap_or(1);
            for (month, count) in &month_activity {
                let bar_len = (*count * 30) / max;
                let bar = "█".repeat(bar_len);
                println!("  {} │{:40} {:>4}", month, bar, count);
            }
        }
    }

    Ok(())
}
