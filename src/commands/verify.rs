//! Comando verify - Verificación completa del proyecto.
//!
//! Ejecuta 21 fases de verificación sobre la documentación.

use crate::errors::OcResult;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::time::{Instant, UNIX_EPOCH};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test file prefixes to exclude from validation
const TEST_PREFIXES: &[&str] = &["TRAP_", "AUTOTEST_", "QUANTUM_TRAP_", "TEST_", "HARDTEST_"];

/// Required YAML fields for full validation
const REQUIRED_YAML_FIELDS: &[&str] = &["id", "title", "parent", "breadcrumb", "type", "status"];

/// Valid document types
const VALID_TYPES: &[&str] = &[
    "hoja",
    "modulo_padre",
    "seccion",
    "contenedor",
    "indice",
    "indice_maestro",
    "especificacion",
    "documento",
    "padre",
    "integracion",
    "testing",
    "feature",
    "estrategia",
    "configuracion",
    "config",
    "perfil",
    "edge_case",
    "arquitectura",
    "seguridad",
    "plugin",
    "optimizacion",
    "infraestructura",
    "esquema",
    "ux",
    "referencia",
    "proceso",
    "planificacion",
    "logica",
    "legal",
    "vision",
    "reglas",
    "programa",
    "privacidad",
    "politica",
    "plantilla",
    "manejo_errores",
    "guia",
    "formulario",
    "flujo",
    "fallback",
    "componente",
    "automatizacion",
    "api",
    "analytics",
    "algoritmo",
    "admin",
    "accesibilidad",
];

/// Valid document statuses
const VALID_STATUSES: &[&str] = &[
    "activo",
    "aceptado",
    "preparado",
    "borrador",
    "pendiente",
    "futuro",
    "deprecado",
    "stub",
    "draft",
    "review",
    "approved",
];

// ═══════════════════════════════════════════════════════════════════════════
// VERIFICATION PHASE
// ═══════════════════════════════════════════════════════════════════════════

/// Fase de verificación individual.
#[derive(Debug, Clone)]
pub struct VerificationPhase {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub passed: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration_ms: u64,
}

impl VerificationPhase {
    pub fn new(id: u8, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            duration_ms: 0,
        }
    }

    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.passed = false;
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    pub fn set_duration(&mut self, ms: u64) {
        self.duration_ms = ms;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// VERIFICATION RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado completo de verificación.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub phases: Vec<VerificationPhase>,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub passed: bool,
    pub duration_ms: u64,
}

impl VerificationResult {
    pub fn new() -> Self {
        Self {
            phases: Vec::new(),
            total_errors: 0,
            total_warnings: 0,
            passed: true,
            duration_ms: 0,
        }
    }

    pub fn add_phase(&mut self, phase: VerificationPhase) {
        self.total_errors += phase.errors.len();
        self.total_warnings += phase.warnings.len();
        if !phase.passed {
            self.passed = false;
        }
        self.phases.push(phase);
    }

    pub fn phases_passed(&self) -> usize {
        self.phases.iter().filter(|p| p.passed).count()
    }

    pub fn phases_failed(&self) -> usize {
        self.phases.len() - self.phases_passed()
    }
}

impl Default for VerificationResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// VERIFY COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de verificación completa.
#[derive(Parser, Debug, Clone)]
#[command(name = "verify", about = "Verificación completa del proyecto")]
pub struct VerifyCommand {
    /// Ruta del proyecto (default: directorio actual).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Modo estricto de schema.
    #[arg(long)]
    pub schema_strict: bool,

    /// Output en formato JSON.
    #[arg(long)]
    pub json: bool,

    /// Ejecutar solo fase específica (número 1-21 o nombre como 'yaml', 'links', etc.).
    #[arg(long)]
    pub phase: Option<String>,

    /// Modo silencioso (solo errores).
    #[arg(short, long)]
    pub quiet: bool,

    /// Modo rápido: omite fases lentas (V16, V17, V19).
    #[arg(short = 'Q', long)]
    pub quick: bool,

    /// P3-B1: Mostrar barra de progreso durante verificación.
    #[arg(long)]
    pub progress: bool,

    /// P2-C1: Usar caché para verificaciones repetidas (sled).
    #[arg(long)]
    pub cache: bool,

    /// RFC-04: Solo procesar archivos en la raíz del directorio (no recursivo).
    #[arg(long)]
    pub root_only: bool,

    /// RFC-04: Patrones de exclusión. Ejemplo: --exclude "_summaries" --exclude "prompts"
    #[arg(long, value_name = "PATTERN")]
    pub exclude: Vec<String>,
}

/// Fases a omitir en modo quick (consumen mucho tiempo)
const SLOW_PHASES: [u8; 3] = [16, 17, 19]; // min_content, placeholders, orphans

/// AN-01 FIX: Parsea fase por número o nombre
fn parse_phase(input: &str) -> Option<u8> {
    // Intenta número directo
    if let Ok(n) = input.parse::<u8>() {
        if (1..=21).contains(&n) {
            return Some(n);
        }
    }
    // Mapea nombres a números
    match input.to_lowercase().as_str() {
        "file_count" | "files" => Some(1),
        "yaml" | "yaml_validation" => Some(2),
        "unique_ids" | "ids" => Some(3),
        "valid_parents" | "parents" => Some(4),
        "breadcrumbs" | "breadcrumb" => Some(5),
        "types" | "type" => Some(6),
        "status" => Some(7),
        "dates_sync" | "dates" => Some(8),
        "internal_links" | "links" => Some(9),
        "embeds" => Some(10),
        "images" => Some(11),
        "code_blocks" | "code" => Some(12),
        "mermaid" => Some(13),
        "tables" => Some(14),
        "headings" => Some(15),
        "min_content" | "content" => Some(16),
        "placeholders" => Some(17),
        "duplicates" => Some(18),
        "orphans" => Some(19),
        "children_count" | "children" => Some(20),
        "hash_integrity" | "hash" => Some(21),
        _ => None,
    }
}

impl VerifyCommand {
    /// Ejecuta la verificación completa.
    pub fn run(&self, data_dir: &PathBuf) -> OcResult<VerificationResult> {
        let start = Instant::now();
        let mut result = VerificationResult::new();

        // Las 21 fases de verificación
        let phase_specs = [
            (1, "file_count", "Conteo de archivos"),
            (2, "yaml_validation", "Validación YAML"),
            (3, "unique_ids", "IDs únicos"),
            (4, "valid_parents", "Parents válidos"),
            (5, "breadcrumbs", "Breadcrumbs consistentes"),
            (6, "types", "Types consistentes"),
            (7, "status", "Status válidos"),
            (8, "dates_sync", "Fechas sincronizadas"),
            (9, "internal_links", "Enlaces internos"),
            (10, "embeds", "Embeds válidos"),
            (11, "images", "Imágenes existentes"),
            (12, "code_blocks", "Código blocks"),
            (13, "mermaid", "Diagramas Mermaid"),
            (14, "tables", "Tablas Markdown"),
            (15, "headings", "Estructura headings"),
            (16, "min_content", "Contenido mínimo"),
            (17, "placeholders", "Placeholders detectados"),
            (18, "duplicates", "Duplicados"),
            (19, "orphans", "Documentos huérfanos"),
            (20, "children_count", "Children count válido"),
            (21, "hash_integrity", "Hash integridad"),
        ];

        for (id, name, desc) in phase_specs.iter() {
            // Skip si se especificó una fase específica
            if let Some(phase_input) = &self.phase {
                if let Some(only_phase) = parse_phase(phase_input) {
                    if *id != only_phase {
                        continue;
                    }
                } else {
                    eprintln!("⚠️ Fase no reconocida: '{}'. Use 1-21 o nombre como 'yaml', 'links', etc.", phase_input);
                    continue;
                }
            }

            // F1.4: Skip fases lentas en modo quick
            if self.quick && SLOW_PHASES.contains(id) {
                if !self.quiet {
                    eprintln!("⏩ V{}: {} (omitida en modo quick)", id, name);
                }
                continue;
            }

            let phase_start = Instant::now();
            let mut phase = VerificationPhase::new(*id, *name, *desc);

            // Ejecutar verificación con data_dir
            self.run_phase(*id, &mut phase, &data_dir);

            phase.set_duration(phase_start.elapsed().as_millis() as u64);
            result.add_phase(phase);
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        Ok(result)
    }

    /// Ejecuta una fase específica.
    fn run_phase(&self, phase_id: u8, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        match phase_id {
            1 => self.phase_file_count(phase, data_dir),
            2 => self.phase_yaml_validation(phase, data_dir),
            3 => self.phase_unique_ids(phase, data_dir),
            4 => self.phase_valid_parents(phase, data_dir),
            5 => self.phase_breadcrumbs(phase, data_dir),
            6 => self.phase_types(phase, data_dir),
            7 => self.phase_status(phase, data_dir),
            8 => self.phase_dates_sync(phase, data_dir),
            9 => self.phase_internal_links(phase, data_dir),
            10 => self.phase_embeds(phase, data_dir),
            11 => self.phase_images(phase, data_dir),
            12 => self.phase_code_blocks(phase, data_dir),
            13 => self.phase_mermaid(phase, data_dir),
            14 => self.phase_tables(phase, data_dir),
            15 => self.phase_headings(phase, data_dir),
            16 => self.phase_min_content(phase, data_dir),
            17 => self.phase_placeholders(phase, data_dir),
            18 => self.phase_duplicates(phase, data_dir),
            19 => self.phase_orphans(phase, data_dir),
            20 => self.phase_children_count(phase, data_dir),
            21 => self.phase_hash_integrity(phase, data_dir),
            _ => {}
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HELPER FUNCTIONS
    // ═══════════════════════════════════════════════════════════════════════

    /// Checks if a file is a test file based on prefix
    fn is_test_file(name: &str) -> bool {
        TEST_PREFIXES.iter().any(|prefix| name.starts_with(prefix))
    }

    /// Extracts a YAML field from file content
    fn get_yaml_field(content: &str, field: &str) -> Option<String> {
        if !content.starts_with("---") {
            return None;
        }

        let end_idx = content[3..].find("---")?;
        let yaml_text = &content[3..3 + end_idx];

        // Simple line-by-line search for field
        for line in yaml_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("{}:", field)) {
                let value_part = trimmed.strip_prefix(&format!("{}:", field))?;
                let value = value_part.trim();
                // Remove surrounding quotes if present
                let cleaned = value.trim_matches(|c| c == '"' || c == '\'');
                if !cleaned.is_empty() {
                    return Some(cleaned.to_string());
                }
            }
        }
        None
    }

    /// Gets all markdown files in directory (excluding test files) - RFC-04 enhanced
    fn get_md_files(data_dir: &PathBuf) -> Vec<PathBuf> {
        Self::get_md_files_with_options(data_dir, false, &[])
    }

    /// RFC-04: Gets markdown files with root_only and exclude options
    fn get_md_files_with_options(data_dir: &PathBuf, root_only: bool, excludes: &[String]) -> Vec<PathBuf> {
        use walkdir::WalkDir;
        
        let mut walker = WalkDir::new(data_dir);
        
        // RFC-04: Si root_only, limitar profundidad a 1
        if root_only {
            walker = walker.max_depth(1);
        }
        
        walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                // Must be a file with .md extension
                if !path.is_file() {
                    return false;
                }
                if path.extension().map_or(true, |ext| ext != "md") {
                    return false;
                }
                // RFC-04: Apply exclude patterns
                let path_str = path.to_string_lossy();
                for pattern in excludes {
                    if path_str.contains(pattern) {
                        return false;
                    }
                }
                // Exclude test files
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    !Self::is_test_file(name)
                } else {
                    false
                }
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 1: FILE COUNT
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_file_count(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        let count = files.len();

        if count == 0 {
            phase.add_error("No se encontraron archivos .md en el directorio");
        }
        // Log count for stats (could add to phase metadata)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 2: YAML VALIDATION
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_yaml_validation(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Skip contextualizador
                if name == "0. Contexualizador.md" {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&path) {
                    // Check if file has YAML frontmatter
                    if !content.starts_with("---") {
                        phase.add_error(format!("{}: Sin YAML frontmatter", name));
                        continue;
                    }

                    // Check if YAML is properly closed
                    if content[3..].find("---").is_none() {
                        phase.add_error(format!("{}: YAML no cerrado (falta '---' final)", name));
                        continue;
                    }

                    // Check required fields
                    let mut missing: Vec<&str> = Vec::new();
                    for field in REQUIRED_YAML_FIELDS {
                        if Self::get_yaml_field(&content, field).is_none() {
                            missing.push(field);
                        }
                    }

                    if !missing.is_empty() {
                        phase.add_error(format!("{}: Falta YAML: {}", name, missing.join(", ")));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 3: UNIQUE IDS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_unique_ids(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        let mut id_files: HashMap<String, Vec<String>> = HashMap::new();

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(id) = Self::get_yaml_field(&content, "id") {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    id_files.entry(id).or_default().push(name);
                }
            }
        }

        for (id, files) in id_files {
            if files.len() > 1 {
                phase.add_error(format!("ID DUPLICADO: '{}' en {}", id, files.join(", ")));
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 4: VALID PARENTS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_valid_parents(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        // First pass: build id_map
        let mut id_map: HashMap<String, PathBuf> = HashMap::new();
        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(id) = Self::get_yaml_field(&content, "id") {
                    id_map.insert(id, path.clone());
                }
            }
        }

        // Second pass: validate parents
        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(parent) = Self::get_yaml_field(&content, "parent") {
                    // Skip root-level docs (parent = 0)
                    if parent == "0" || parent.is_empty() {
                        continue;
                    }

                    if !id_map.contains_key(&parent) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown");
                        phase.add_error(format!("{}: Parent '{}' no existe", name, parent));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 5: BREADCRUMBS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_breadcrumbs(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let id = Self::get_yaml_field(&content, "id");
                let breadcrumb = Self::get_yaml_field(&content, "breadcrumb");

                if let (Some(id), Some(bc)) = (id, breadcrumb) {
                    // Check if id is contained in breadcrumb
                    if !bc.contains(&id) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown");
                        phase.add_warning(format!(
                            "{}: Breadcrumb inconsistente (ID '{}' no en '{}')",
                            name, id, bc
                        ));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 6: TYPES
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_types(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(doc_type) = Self::get_yaml_field(&content, "type") {
                    let type_lower = doc_type.to_lowercase();
                    if !VALID_TYPES.contains(&type_lower.as_str()) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown");
                        phase.add_warning(format!("{}: Type no estándar: '{}'", name, doc_type));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 7: STATUS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_status(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(status) = Self::get_yaml_field(&content, "status") {
                    let status_lower = status.to_lowercase();
                    if !VALID_STATUSES.contains(&status_lower.as_str()) {
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown");
                        phase.add_warning(format!("{}: Status no estándar: '{}'", name, status));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 8: DATES SYNC (CRITICAL - detects 832+ anomalies)
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_dates_sync(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            // Skip contextualizador
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("0.") {
                    continue;
                }
            }

            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(yaml_date) = Self::get_yaml_field(&content, "last_updated") {
                    // Get file modification time
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(mtime) = metadata.modified() {
                            if let Ok(duration) = mtime.duration_since(UNIX_EPOCH) {
                                let fs_secs = duration.as_secs();

                                // Parse YAML date: "YYYY-MM-DD HH:MM" or "YYYY-MM-DD"
                                if let Some(yaml_secs) = Self::parse_date_to_secs(&yaml_date) {
                                    // Difference in minutes
                                    let diff_secs = fs_secs.abs_diff(yaml_secs);
                                    let diff_minutes = diff_secs / 60;

                                    // Threshold: 24 hours (1440 minutes)
                                    if diff_minutes > 1440 {
                                        let name = path
                                            .file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("unknown");
                                        phase.add_warning(format!(
                                            "{}: YAML date '{}' vs file mtime (>24h drift)",
                                            name, yaml_date
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// RFC-01: Parse date string to seconds since UNIX_EPOCH (timezone-aware)
    fn parse_date_to_secs(date_str: &str) -> Option<u64> {
        use chrono::{Local, NaiveDateTime, TimeZone};
        
        // Format: "YYYY-MM-DD HH:MM:SS" or "YYYY-MM-DD HH:MM" or "YYYY-MM-DD"
        let cleaned = date_str.trim().trim_matches('"');
        
        // Try full datetime formats
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%d %H:%M:%S") {
            if let Some(local_dt) = Local.from_local_datetime(&naive).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, "%Y-%m-%d %H:%M") {
            if let Some(local_dt) = Local.from_local_datetime(&naive).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        // Try date-only format (assume midnight local time)
        if let Ok(naive) = chrono::NaiveDate::parse_from_str(cleaned, "%Y-%m-%d") {
            let naive_dt = naive.and_hms_opt(0, 0, 0)?;
            if let Some(local_dt) = Local.from_local_datetime(&naive_dt).single() {
                return Some(local_dt.timestamp() as u64);
            }
        }
        
        None
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 9: INTERNAL LINKS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_internal_links(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        // Build file map for fuzzy matching
        let mut file_map: HashMap<String, String> = HashMap::new();
        for path in &files {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                file_map.insert(stem.to_lowercase(), stem.to_string());
            }
        }

        use crate::core::patterns::RE_WIKI_LINK_WITH_ALIAS;
        let link_re = &*RE_WIKI_LINK_WITH_ALIAS;

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                for cap in link_re.captures_iter(&content) {
                    let link = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    let link = link.trim().trim_end_matches('\\');

                    // Skip external links, anchors, special patterns
                    if link.is_empty()
                        || link.starts_with("http")
                        || link.starts_with('#')
                        || link.contains("Métrica")
                        || link.contains("Valor")
                    {
                        continue;
                    }

                    // Handle anchors: [[File#Section]]
                    let link_file = if link.contains('#') {
                        link.split('#').next().unwrap_or(link)
                    } else {
                        link
                    };

                    // Check if target exists
                    let target = data_dir.join(format!("{}.md", link_file));
                    if !target.exists() {
                        // Try case-insensitive match
                        let link_lower = link_file.to_lowercase();
                        if let Some(correct_name) = file_map.get(&link_lower) {
                            phase.add_error(format!(
                                "{}: CASE-SENSITIVE [[{}]] -> debería ser [[{}]]",
                                name, link, correct_name
                            ));
                        } else {
                            // Only report truly broken links (not internal anchors or mentions)
                            if !link.starts_with('@') && !link.contains('_') {
                                phase.add_warning(format!("{}: Link roto [[{}]]", name, link));
                            }
                        }
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 10: EMBEDS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_embeds(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        use crate::core::patterns::RE_OBSIDIAN_EMBED;
        let embed_re = &*RE_OBSIDIAN_EMBED;

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                for cap in embed_re.captures_iter(&content) {
                    let embed = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    let embed = embed.trim().trim_end_matches('\\');

                    if embed.is_empty() {
                        continue;
                    }

                    // Check if embedded file exists
                    let target = data_dir.join(format!("{}.md", embed));
                    if !target.exists() {
                        phase.add_warning(format!("{}: Embed no existe ![[{}]]", name, embed));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 11: IMAGES
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_images(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        use crate::core::patterns::RE_IMAGE;
        let img_re = &*RE_IMAGE;

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                for cap in img_re.captures_iter(&content) {
                    let img_path = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                    // Skip external images
                    if img_path.starts_with("http") {
                        continue;
                    }

                    // Check if image exists
                    let target = data_dir.join(img_path);
                    if !target.exists() {
                        phase.add_warning(format!("{}: Imagen no existe: {}", name, img_path));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 12: CODE BLOCKS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_code_blocks(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                // Count opening and closing code fences
                let open_count = content.matches("```").count();

                // Code blocks must be paired
                if open_count % 2 != 0 {
                    phase.add_warning(format!(
                        "{}: Code block no cerrado ({} delimitadores)",
                        name, open_count
                    ));
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 13: MERMAID
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_mermaid(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        use crate::core::patterns::RE_MERMAID;
        let mermaid_re = &*RE_MERMAID;

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                for cap in mermaid_re.captures_iter(&content) {
                    let mermaid_content = cap.get(1).map(|m| m.as_str()).unwrap_or("");

                    // Basic validation: check for diagram type
                    let has_type = mermaid_content.contains("graph")
                        || mermaid_content.contains("flowchart")
                        || mermaid_content.contains("sequenceDiagram")
                        || mermaid_content.contains("classDiagram")
                        || mermaid_content.contains("stateDiagram")
                        || mermaid_content.contains("pie")
                        || mermaid_content.contains("gantt")
                        || mermaid_content.contains("erDiagram");

                    if !has_type && !mermaid_content.trim().is_empty() {
                        phase.add_warning(format!("{}: Mermaid sin tipo de diagrama válido", name));
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 14: TABLES
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_tables(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                let lines: Vec<&str> = content.lines().collect();
                let mut in_table = false;
                let mut table_start_line = 0;
                let mut has_separator = false;

                for (i, line) in lines.iter().enumerate() {
                    let trimmed = line.trim();

                    if trimmed.starts_with('|') && trimmed.ends_with('|') {
                        if !in_table {
                            in_table = true;
                            table_start_line = i;
                            has_separator = false;
                        }

                        // Check for separator line (|---|---|)
                        if trimmed.contains("---")
                            || trimmed.contains(":--")
                            || trimmed.contains("--:")
                            || trimmed.contains(":-:")
                        {
                            has_separator = true;
                        }
                    } else if in_table {
                        // End of table
                        if !has_separator {
                            phase.add_warning(format!(
                                "{}: Tabla en línea {} sin separador de header",
                                name,
                                table_start_line + 1
                            ));
                        }
                        in_table = false;
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 15: HEADINGS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_headings(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                // Count H1 headings (# at start of line, not ##)
                let h1_count = content
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim();
                        trimmed.starts_with("# ") && !trimmed.starts_with("## ")
                    })
                    .count();

                if h1_count == 0 {
                    // Skip files without H1 entirely (YAML title may be enough)
                } else if h1_count > 1 {
                    phase.add_warning(format!("{}: Múltiples H1 ({} encontrados)", name, h1_count));
                }

                // Check for heading hierarchy issues
                let mut last_level = 0u8;
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with('#') && !trimmed.starts_with("```") {
                        let level = trimmed.chars().take_while(|c| *c == '#').count() as u8;
                        if last_level > 0 && level > last_level + 1 {
                            phase.add_warning(format!(
                                "{}: Salto de heading H{} a H{}",
                                name, last_level, level
                            ));
                            break; // Only report once per file
                        }
                        last_level = level;
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 16: MIN CONTENT
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_min_content(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);
        const MIN_WORDS: usize = 50;

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                // Skip YAML frontmatter
                let body = if content.starts_with("---") {
                    if let Some(end) = content[3..].find("---") {
                        &content[3 + end + 3..]
                    } else {
                        &content
                    }
                } else {
                    &content
                };

                // Count words (simple split on whitespace)
                let word_count = body.split_whitespace().count();

                if word_count < MIN_WORDS {
                    phase.add_warning(format!(
                        "{}: Contenido mínimo ({} palabras, mínimo {})",
                        name, word_count, MIN_WORDS
                    ));
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 17: PLACEHOLDERS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_placeholders(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        const PLACEHOLDER_PATTERNS: &[&str] = &[
            "TBD",
            "TODO",
            "FIXME",
            "XXX",
            "PENDING",
            "[PENDIENTE]",
            "[TODO]",
            "[TBD]",
            "Lorem ipsum",
            "placeholder",
            "PLACEHOLDER",
            "Contenido pendiente",
            "Por definir",
        ];

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                for pattern in PLACEHOLDER_PATTERNS {
                    if content.contains(pattern) {
                        phase
                            .add_warning(format!("{}: Placeholder detectado: '{}'", name, pattern));
                        break; // Only report first placeholder per file
                    }
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 18: DUPLICATES (Potential duplicate files)
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_duplicates(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        // Group files by title
        let mut title_map: HashMap<String, Vec<String>> = HashMap::new();

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(title) = Self::get_yaml_field(&content, "title") {
                    let title_lower = title.to_lowercase();
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    title_map.entry(title_lower).or_default().push(name);
                }
            }
        }

        // Report duplicates
        for (title, files) in title_map {
            if files.len() > 1 {
                phase.add_warning(format!(
                    "Título duplicado '{}' en: {}",
                    title,
                    files.join(", ")
                ));
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 19: ORPHANS
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_orphans(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        // Build set of all references
        let mut all_refs: HashSet<String> = HashSet::new();
        use crate::core::patterns::RE_WIKI_LINK_WITH_ALIAS;
        let link_re = &*RE_WIKI_LINK_WITH_ALIAS;

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                for cap in link_re.captures_iter(&content) {
                    if let Some(m) = cap.get(1) {
                        all_refs.insert(m.as_str().to_lowercase());
                    }
                }
            }
        }

        // Check each file
        for path in &files {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Skip contextualizador
            if name.starts_with("0.") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                // Check if has valid parent
                if let Some(parent) = Self::get_yaml_field(&content, "parent") {
                    if parent != "0" && !parent.is_empty() {
                        continue; // Has parent = not orphan
                    }
                }

                // Check if referenced anywhere
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let stem_lower = stem.to_lowercase();

                if !all_refs.contains(&stem_lower) {
                    phase.add_warning(format!("HUÉRFANO: {}", name));
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 20: CHILDREN COUNT
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_children_count(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        let files = Self::get_md_files(data_dir);

        // Build parent -> children map
        let mut children_of: HashMap<String, Vec<String>> = HashMap::new();

        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(file_id) = Self::get_yaml_field(&content, "id") {
                    if let Some(parent_id) = Self::get_yaml_field(&content, "parent") {
                        if parent_id != "0" && !parent_id.is_empty() {
                            children_of.entry(parent_id).or_default().push(file_id);
                        }
                    }
                }
            }
        }

        // Check each file with children_count field
        for path in &files {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(cc_str) = Self::get_yaml_field(&content, "children_count") {
                    if let Ok(expected) = cc_str.parse::<usize>() {
                        if let Some(file_id) = Self::get_yaml_field(&content, "id") {
                            let actual = children_of.get(&file_id).map(|v| v.len()).unwrap_or(0);

                            if actual != expected {
                                let name = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown");
                                phase.add_warning(format!(
                                    "{}: children_count={} vs actual={}",
                                    name, expected, actual
                                ));
                            }
                        }
                    }
                }
            }
        }
    }



    // ═══════════════════════════════════════════════════════════════════════
    // PHASE 21: HASH INTEGRITY
    // ═══════════════════════════════════════════════════════════════════════

    fn phase_hash_integrity(&self, phase: &mut VerificationPhase, data_dir: &PathBuf) {
        use sha2::{Digest, Sha256};
        
        let files = Self::get_md_files(data_dir);

        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                // Check if file has stored hash
                if let Some(stored_hash) = Self::get_yaml_field(&content, "content_hash") {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    // RFC-06: Usar exactamente la misma lógica de hash que sync.rs
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
                    let computed_hex = format!("{:x}", hasher.finalize())[..16].to_string();

                    // Compare stored vs computed
                    if stored_hash.trim().trim_matches('"') != computed_hex {
                        phase.add_warning(format!("{}: Hash mismatch (stored vs computed)", name));
                    }
                }
            }
        }
    }

    /// Exit code basado en resultado.
    pub fn exit_code(result: &VerificationResult) -> i32 {
        if result.passed {
            0
        } else if result.total_errors > 0 {
            1
        } else {
            2 // warnings only
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_phase_new() {
        let phase = VerificationPhase::new(1, "test", "Test phase");
        assert!(phase.passed);
        assert!(phase.errors.is_empty());
    }

    #[test]
    fn test_phase_add_error() {
        let mut phase = VerificationPhase::new(1, "test", "Test");
        phase.add_error("something failed");

        assert!(!phase.passed);
        assert_eq!(phase.errors.len(), 1);
    }

    #[test]
    fn test_verification_result() {
        let mut result = VerificationResult::new();

        let mut phase1 = VerificationPhase::new(1, "p1", "d1");
        phase1.add_error("error");

        let phase2 = VerificationPhase::new(2, "p2", "d2");

        result.add_phase(phase1);
        result.add_phase(phase2);

        assert_eq!(result.phases_passed(), 1);
        assert_eq!(result.phases_failed(), 1);
    }

    #[test]
    fn test_exit_code() {
        let result = VerificationResult::new();
        assert_eq!(VerifyCommand::exit_code(&result), 0);
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: VerifyCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let data_dir = cmd
        .path
        .clone()
        .unwrap_or_else(|| PathBuf::from(&cli.data_dir));
    let result = cmd.run(&data_dir)?;

    if cmd.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "passed": result.passed,
                "phases_total": result.phases.len(),
                "phases_passed": result.phases_passed(),
                "errors": result.total_errors,
                "warnings": result.total_warnings,
                "duration_ms": result.duration_ms
            }))?
        );
    } else {
        // FIX NUCLEAR C1: Imprimir CADA error y warning detalladamente
        for phase in &result.phases {
            let status = if phase.passed { "✅" } else { "❌" };
            println!(
                "{} Fase {}: {} ({}ms)",
                status, phase.id, phase.name, phase.duration_ms
            );
            
            // Imprimir errores con color rojo
            for error in &phase.errors {
                println!("   \x1b[31m✗ ERROR:\x1b[0m {}", error);
            }
            
            // Imprimir warnings con color amarillo
            for warning in &phase.warnings {
                println!("   \x1b[33m⚠ WARNING:\x1b[0m {}", warning);
            }
        }
        println!(
            "\n📊 {}/{} fases pasaron, {} errores, {} warnings",
            result.phases_passed(),
            result.phases.len(),
            result.total_errors,
            result.total_warnings
        );
    }


    std::process::exit(VerifyCommand::exit_code(&result));
}
