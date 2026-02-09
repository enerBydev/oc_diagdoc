//! Comando lint - Análisis estático de documentación.
//!
//! Detecta problemas de estilo y estructura.

use crate::errors::OcResult;
use clap::Parser;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// LINT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Severidad de problema lint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Un problema de lint.
#[derive(Debug, Clone)]
pub struct LintIssue {
    pub code: String,
    pub message: String,
    pub file: PathBuf,
    pub line: Option<usize>,
    pub severity: LintSeverity,
    pub fixable: bool,
}

impl LintIssue {
    pub fn error(code: &str, message: &str, file: PathBuf) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            file,
            line: None,
            severity: LintSeverity::Error,
            fixable: false,
        }
    }

    pub fn warning(code: &str, message: &str, file: PathBuf) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            file,
            line: None,
            severity: LintSeverity::Warning,
            fixable: false,
        }
    }
}

/// Resultado del lint.
#[derive(Debug, Clone)]
pub struct LintResult {
    pub issues: Vec<LintIssue>,
    pub files_checked: usize,
    pub files_with_issues: usize,
}

impl LintResult {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            files_checked: 0,
            files_with_issues: 0,
        }
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == LintSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == LintSeverity::Warning)
            .count()
    }

    pub fn fixable_count(&self) -> usize {
        self.issues.iter().filter(|i| i.fixable).count()
    }

    pub fn is_clean(&self) -> bool {
        self.error_count() == 0 && self.warning_count() == 0
    }
}

impl Default for LintResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LINT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de lint.
#[derive(Parser, Debug, Clone)]
#[command(name = "lint", about = "Análisis estático de documentación")]
pub struct LintCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Corregir automáticamente problemas fixables.
    #[arg(long)]
    pub fix: bool,

    /// Modo dry-run: mostrar cambios sin aplicar (requiere --fix).
    #[arg(long)]
    pub dry_run: bool,

    /// Solo errores, omitir warnings/hints.
    #[arg(long)]
    pub errors_only: bool,

    /// Output formato JSON.
    #[arg(long)]
    pub json: bool,

    // L4: Flags avanzados
    /// Ejecutar solo regla específica (ej: L001, L003).
    #[arg(long, value_name = "RULE")]
    pub rule: Option<String>,

    /// Mostrar estadísticas por categoría.
    #[arg(long)]
    pub summary: bool,

    /// P3-A3: Mostrar detalle de todos los fixes aplicables.
    #[arg(long)]
    pub show_fixes: bool,

    /// RFC-03: Explicar regla de lint (ej: --explain L006).
    #[arg(long, value_name = "CODE")]
    pub explain: Option<String>,
}

impl LintCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<LintResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use std::collections::HashSet;

        // RFC-03: Si se pidió --explain, mostrar documentación y salir
        if let Some(code) = &self.explain {
            crate::core::lint_docs::print_rule_explanation(code);
            return Ok(LintResult::new());
        }

        let mut result = LintResult::new();
        let mut files_fixed = 0usize;

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        result.files_checked = files.len();
        let mut files_with_issues_set: HashSet<PathBuf> = HashSet::new();

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                // L4.4: Aplicar --fix si se solicitó
                if self.fix {
                    if let Some(fixed_content) = self.fix_file(file_path, &content) {
                        if self.dry_run {
                            eprintln!("🔍 [DRY-RUN] Sería corregido: {}", file_path.display());
                        } else {
                            if std::fs::write(file_path, &fixed_content).is_ok() {
                                files_fixed += 1;
                            }
                        }
                    }
                }

                let issues = self.lint_file(file_path, &content, data_dir);

                if !issues.is_empty() {
                    files_with_issues_set.insert(file_path.clone());
                    for issue in issues {
                        if self.errors_only && issue.severity != LintSeverity::Error {
                            continue;
                        }
                        result.issues.push(issue);
                    }
                }
            }
        }

        result.files_with_issues = files_with_issues_set.len();

        // Agregar estadística de archivos corregidos (usar info log si hay fix)
        if self.fix && files_fixed > 0 {
            eprintln!("✅ {} archivos corregidos automáticamente", files_fixed);
        }

        Ok(result)
    }

    /// Aplica todas las reglas a un archivo.
    fn lint_file(&self, file_path: &PathBuf, content: &str, data_dir: &std::path::Path) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Regla 1: Archivo debe tener frontmatter YAML
        if self.should_run_rule("L001") {
            issues.extend(self.rule_frontmatter(file_path, content));
        }

        // Regla 2: Headers deben ser jerárquicos
        if self.should_run_rule("L002") {
            issues.extend(self.rule_header_hierarchy(file_path, &lines));
        }

        // Regla 3: No trailing whitespace
        if self.should_run_rule("L003") {
            issues.extend(self.rule_trailing_whitespace(file_path, &lines));
        }

        // Regla 4: Archivo termina con newline
        if self.should_run_rule("L004") {
            issues.extend(self.rule_final_newline(file_path, content));
        }

        // Regla 5: No líneas > 300 caracteres (muy largas)
        if self.should_run_rule("L005") {
            issues.extend(self.rule_line_length(file_path, &lines));
        }

        // Regla 6: Code blocks deben tener lenguaje
        if self.should_run_rule("L006") {
            issues.extend(self.rule_code_block_language(file_path, &lines));
        }

        // Regla 7: Headers no duplicados
        if self.should_run_rule("L007") {
            issues.extend(self.rule_duplicate_headers(file_path, &lines));
        }

        // Regla 8: Frontmatter fields obligatorios
        if self.should_run_rule("L008") {
            issues.extend(self.rule_required_fields(file_path, content));
        }

        // L4: Regla 9: Tablas con header
        if self.should_run_rule("L009") {
            issues.extend(self.rule_table_headers(file_path, &lines));
        }

        // L4: Regla 10: Imágenes con alt text
        if self.should_run_rule("L010") {
            issues.extend(self.rule_image_alt(file_path, &lines));
        }

        // L011: Separadores duplicados en tablas
        if self.should_run_rule("L011") {
            issues.extend(self.rule_table_double_separator(file_path, &lines));
        }

        // L012: Pipes sin escapar en wikilinks dentro de tablas
        if self.should_run_rule("L012") {
            issues.extend(self.rule_unescaped_pipe_in_table(file_path, &lines));
        }

        // L013: Nietos count mismatch
        if self.should_run_rule("L013") {
            issues.extend(self.rule_nietos_mismatch(file_path, &lines, data_dir));
        }

        // L014: Wikilinks con paths absolutos
        if self.should_run_rule("L014") {
            issues.extend(self.rule_wikilink_absolute_path(file_path, &lines));
        }

        issues
    }


    /// Regla: Archivo debe tener frontmatter YAML.
    fn rule_frontmatter(&self, file_path: &PathBuf, content: &str) -> Vec<LintIssue> {
        // FIX #32: Excluir archivos en _summaries/ (no requieren frontmatter)
        let path_str = file_path.to_string_lossy();
        if path_str.contains("_summaries/") || path_str.contains("_summaries\\") {
            return Vec::new();
        }

        if !content.starts_with("---") {
            return vec![LintIssue {
                code: "L001".to_string(),
                message: "Archivo sin frontmatter YAML".to_string(),
                file: file_path.clone(),
                line: Some(1),
                severity: LintSeverity::Warning,
                fixable: false,
            }];
        }
        Vec::new()
    }

    /// Regla: Headers deben ser jerárquicos (no saltar niveles).
    fn rule_header_hierarchy(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let mut last_level = 0;
        let mut in_code_block = false;  // FIX BUG L002: Agregar tracking de code blocks

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            
            // FIX BUG L002: Detectar inicio/fin de code blocks
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = !in_code_block;
                continue;
            }
            
            // FIX BUG L002: Skip líneas dentro de code blocks
            if in_code_block {
                continue;
            }

            if line.starts_with('#') {
                let level = line.chars().take_while(|c| *c == '#').count();

                // No debe saltar más de 1 nivel
                if last_level > 0 && level > last_level + 1 {
                    issues.push(LintIssue {
                        code: "L002".to_string(),
                        message: format!("Header salta de H{} a H{}", last_level, level),
                        file: file_path.clone(),
                        line: Some(idx + 1),
                        severity: LintSeverity::Warning,
                        fixable: false,
                    });
                }
                last_level = level;
            }
        }
        issues
    }

    /// Regla: No trailing whitespace.
    fn rule_trailing_whitespace(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                issues.push(LintIssue {
                    code: "L003".to_string(),
                    message: "Trailing whitespace".to_string(),
                    file: file_path.clone(),
                    line: Some(idx + 1),
                    severity: LintSeverity::Info,
                    fixable: true,
                });
            }
        }
        issues
    }

    /// Regla: Archivo termina con newline.
    fn rule_final_newline(&self, file_path: &PathBuf, content: &str) -> Vec<LintIssue> {
        if !content.ends_with('\n') {
            return vec![LintIssue {
                code: "L004".to_string(),
                message: "Archivo no termina con newline".to_string(),
                file: file_path.clone(),
                line: None,
                severity: LintSeverity::Info,
                fixable: true,
            }];
        }
        Vec::new()
    }

    /// Regla: Líneas no muy largas.
    fn rule_line_length(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        // FIX #33: Aumentar umbral de 300 a 800 chars
        const MAX_LINE_LENGTH: usize = 800;
        
        let mut issues = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            if line.len() > MAX_LINE_LENGTH {
                issues.push(LintIssue {
                    code: "L005".to_string(),
                    message: format!("Línea muy larga ({} chars, max: {})", line.len(), MAX_LINE_LENGTH),
                    file: file_path.clone(),
                    line: Some(idx + 1),
                    severity: LintSeverity::Warning,
                    fixable: false,
                });
            }
        }
        issues
    }

    /// Regla: Code blocks deben tener lenguaje especificado.
    /// RFC-28: Fixed bug - ahora distingue aperturas vs cierres de code blocks
    fn rule_code_block_language(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let mut in_code_block = false;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Detectar líneas que empiezan con ```
            if trimmed.starts_with("```") {
                if !in_code_block {
                    // APERTURA de code block
                    in_code_block = true;
                    
                    // Verificar si tiene lenguaje especificado después de ```
                    let after_backticks = trimmed.trim_start_matches('`');
                    if after_backticks.is_empty() {
                        // Solo "```" sin lenguaje = problema real
                        issues.push(LintIssue {
                            code: "L006".to_string(),
                            message: "Code block sin lenguaje especificado".to_string(),
                            file: file_path.clone(),
                            line: Some(idx + 1),
                            severity: LintSeverity::Hint,
                            fixable: false,
                        });
                    }
                } else {
                    // CIERRE de code block - NO reportar L006
                    in_code_block = false;
                }
            }
        }
        issues
    }

    /// Regla: Headers no duplicados.
    /// RFC-FIX: Implementado tracking de estado in_code_block para ignorar 
    /// shebangs, comentarios y contenido dentro de bloques de código fenced.
    fn rule_duplicate_headers(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use std::collections::HashMap;
        let mut issues = Vec::new();
        let mut seen: HashMap<String, usize> = HashMap::new();
        let mut in_code_block = false; // Track estado de fenced code blocks

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Toggle estado de code blocks (soporta ``` y ~~~)
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = !in_code_block;
                continue;
            }
            
            // Ignorar contenido dentro de code blocks
            if in_code_block {
                continue;
            }
            
            // Solo procesar headers Markdown válidos (# seguido de espacio y texto)
            if line.starts_with('#') {
                // Un header válido tiene al menos un # seguido de espacio
                let hash_count = line.chars().take_while(|c| *c == '#').count();
                let remaining = &line[hash_count..];
                
                // Skip si no hay espacio después de los # (ej: #hashtag, #!/bin/bash)
                if !remaining.starts_with(' ') {
                    continue;
                }
                
                let header = remaining.trim().to_lowercase();
                
                // Protección adicional contra shebangs (edge case)
                if header.starts_with('!') {
                    continue;
                }
                
                // Skip headers vacíos
                if header.is_empty() {
                    continue;
                }
                
                if let Some(prev_line) = seen.get(&header) {
                    issues.push(LintIssue {
                        code: "L007".to_string(),
                        message: format!(
                            "Header duplicado (primera aparición línea {})",
                            prev_line
                        ),
                        file: file_path.clone(),
                        line: Some(idx + 1),
                        severity: LintSeverity::Warning,
                        fixable: false,
                    });
                } else {
                    seen.insert(header, idx + 1);
                }
            }
        }
        issues
    }

    /// Regla: Campos obligatorios en frontmatter.
    fn rule_required_fields(&self, file_path: &PathBuf, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let required = ["id:", "title:"];

        // Solo revisar si tiene frontmatter
        if content.starts_with("---") {
            for field in required {
                if !content.contains(field) {
                    issues.push(LintIssue {
                        code: "L008".to_string(),
                        message: format!(
                            "Campo obligatorio faltante: {}",
                            field.trim_end_matches(':')
                        ),
                        file: file_path.clone(),
                        line: None,
                        severity: LintSeverity::Error,
                        fixable: false,
                    });
                }
            }
        }
        issues
    }

    // ═══════════════════════════════════════════════════════════════════════
    // L4: REGLAS AVANZADAS
    // ═══════════════════════════════════════════════════════════════════════

    /// L4.2 Regla: Tablas deben tener fila de header.
    fn rule_table_headers(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use crate::core::patterns::{RE_TABLE_ROW, RE_TABLE_SEPARATOR};
        let table_row = &*RE_TABLE_ROW;
        let separator_row = &*RE_TABLE_SEPARATOR;

        let mut issues = Vec::new();
        let mut i = 0;
        let mut in_code_block = false;  // FIX BUG L009: Agregar tracking de code blocks

        while i < lines.len() {
            let trimmed = lines[i].trim();
            
            // FIX BUG L009: Detectar inicio/fin de code blocks
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                in_code_block = !in_code_block;
                i += 1;
                continue;
            }
            
            // FIX BUG L009: Skip líneas dentro de code blocks
            if in_code_block {
                i += 1;
                continue;
            }

            if table_row.is_match(trimmed) {
                // Verificar que la siguiente línea sea separador
                if i + 1 >= lines.len() || !separator_row.is_match(lines[i + 1].trim()) {
                    issues.push(LintIssue {
                        code: "L009".to_string(),
                        message: "Tabla sin fila de header/separador".to_string(),
                        file: file_path.clone(),
                        line: Some(i + 1),
                        severity: LintSeverity::Warning,
                        fixable: false,
                    });
                }
                // Saltar hasta el final de la tabla
                while i < lines.len() && table_row.is_match(lines[i].trim()) {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        issues
    }

    /// L4.3 Regla: Imágenes deben tener alt text.
    fn rule_image_alt(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use crate::core::patterns::RE_IMAGE_EMPTY_ALT;
        // Busca ![](path) donde alt text está vacío
        let empty_alt = &*RE_IMAGE_EMPTY_ALT;

        let mut issues = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            if empty_alt.is_match(line) {
                issues.push(LintIssue {
                    code: "L010".to_string(),
                    message: "Imagen sin alt text".to_string(),
                    file: file_path.clone(),
                    line: Some(idx + 1),
                    severity: LintSeverity::Warning,
                    fixable: false,
                });
            }
        }
        issues
    }

    /// L011: Detecta separadores duplicados en tablas.
    /// Una tabla válida solo tiene UN separador |---| después del header.
    fn rule_table_double_separator(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use crate::core::patterns::{RE_TABLE_ROW, RE_TABLE_SEPARATOR};
        let table_row = &*RE_TABLE_ROW;
        let separator_row = &*RE_TABLE_SEPARATOR;

        let mut issues = Vec::new();
        let mut i = 0;
        
        while i < lines.len() {
            let trimmed = lines[i].trim();
            
            // Detectar inicio de tabla (línea que es una fila de tabla)
            if table_row.is_match(trimmed) && !separator_row.is_match(trimmed) {
                // Estamos en una tabla
                let table_start = i;
                let mut separator_count = 0;
                let mut first_separator_line = 0;
                
                // Recorrer la tabla completa
                while i < lines.len() {
                    let line_trimmed = lines[i].trim();
                    
                    // ¿Es fila de tabla o separador?
                    if separator_row.is_match(line_trimmed) {
                        separator_count += 1;
                        if separator_count == 1 {
                            first_separator_line = i;
                        } else {
                            // Separador adicional = problema
                            issues.push(LintIssue {
                                code: "L011".to_string(),
                                message: format!(
                                    "Separador duplicado en tabla (primer sep línea {})",
                                    first_separator_line + 1
                                ),
                                file: file_path.clone(),
                                line: Some(i + 1),
                                severity: LintSeverity::Error,
                                fixable: true,
                            });
                        }
                        i += 1;
                    } else if table_row.is_match(line_trimmed) {
                        i += 1;
                    } else {
                        // Fin de tabla
                        break;
                    }
                }
            } else {
                i += 1;
            }
        }
        issues
    }

    /// L012: Detecta wikilinks con pipes sin escapar dentro de tablas.
    /// En tablas markdown, [[X|Y]] debe ser [[X\|Y]] para no romper columnas.
    fn rule_unescaped_pipe_in_table(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use regex::Regex;
        lazy_static::lazy_static! {
            // Detecta [[...pipes sin escapar...]] - pipe que NO está precedido por \
            static ref WIKILINK_UNESCAPED: Regex = Regex::new(r"\[\[([^\]\|\\]+)\|([^\]]+)\]\]").unwrap();
        }
        
        let mut issues = Vec::new();
        
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            // Solo analizar líneas de tabla (empiezan con |)
            if trimmed.starts_with('|') {
                // Buscar wikilinks con pipes sin escapar
                for cap in WIKILINK_UNESCAPED.captures_iter(line) {
                    let full_match = cap.get(0).unwrap().as_str();
                    // Verificar que no esté escapado (no hay \ antes del |)
                    if !full_match.contains(r"\|") {
                        issues.push(LintIssue {
                            code: "L012".to_string(),
                            message: format!("Wikilink con pipe sin escapar: {}", full_match),
                            file: file_path.clone(),
                            line: Some(idx + 1),
                            severity: LintSeverity::Error,
                            fixable: true,
                        });
                    }
                }
            }
        }
        issues
    }

    /// L013: Detecta columna Nietos con valor incorrecto.
    /// Compara el valor en la tabla con el conteo real de archivos descendientes.
    fn rule_nietos_mismatch(&self, file_path: &PathBuf, lines: &[&str], data_dir: &std::path::Path) -> Vec<LintIssue> {
        use regex::Regex;
        lazy_static::lazy_static! {
            // Detectar tablas con columna Nietos
            static ref NIETOS_HEADER: Regex = Regex::new(r"\|\s*Nietos\s*\|").unwrap();
            // Extraer wikilink de la primera columna
            static ref WIKILINK: Regex = Regex::new(r"\[\[([^\]\|]+)").unwrap();
        }
        
        let mut issues = Vec::new();
        
        // Buscar tablas con columna Nietos
        for (idx, line) in lines.iter().enumerate() {
            if NIETOS_HEADER.is_match(line) {
                // Encontramos header de tabla con Nietos
                // Buscar índice de columna Nietos
                let parts: Vec<&str> = line.split('|').collect();
                let mut nietos_col_idx = 0;
                for (col_idx, part) in parts.iter().enumerate() {
                    if part.trim().contains("Nietos") {
                        nietos_col_idx = col_idx;
                        break;
                    }
                }
                
                // Procesar filas de datos (después del separador)
                let mut row_idx = idx + 2; // Saltar header y separador
                while row_idx < lines.len() {
                    let row = lines[row_idx].trim();
                    if !row.starts_with('|') {
                        break;
                    }
                    
                    // Enmascarar pipes dentro de wikilinks
                    let masked = row.replace(r"\|", "§PIPE§");
                    let cols: Vec<&str> = masked.split('|').collect();
                    
                    if cols.len() > nietos_col_idx {
                        let nietos_str = cols[nietos_col_idx].trim().replace("§PIPE§", r"\|");
                        
                        // Extraer wikilink de la primera columna
                        let id_col = cols.get(1).unwrap_or(&"");
                        if let Some(cap) = WIKILINK.captures(id_col) {
                            let link_target_raw = cap.get(1).unwrap().as_str();
                            // Limpiar §PIPE§ y obtener solo el nombre del archivo
                            let link_target_clean = link_target_raw.replace("§PIPE§", "|");
                            let link_target = link_target_clean.split('|').next().unwrap_or(link_target_raw);
                            
                            // FIX L013: Extraer ID numérico (e.g., "3.1.7" de "3.1.7 sistema_costos")
                            let child_id = link_target.split(' ').next().unwrap_or(link_target).trim_end_matches('.');
                            
                            // Contar descendientes reales en filesystem
                            let actual_count = self.count_descendants(data_dir, child_id);
                            
                            // Parsear valor reclamado en la tabla
                            let claimed: usize = nietos_str.parse().unwrap_or(0);
                            
                            // Solo reportar si hay discrepancia REAL
                            if actual_count != claimed {
                                issues.push(LintIssue {
                                    code: "L013".to_string(),
                                    message: format!("Nietos={} incorrecto (real={}) para {}", claimed, actual_count, link_target),
                                    file: file_path.clone(),
                                    line: Some(row_idx + 1),
                                    severity: LintSeverity::Warning,
                                    fixable: true,
                                });
                            }
                        }
                    }
                    row_idx += 1;
                }
            }
        }
        issues
    }
    
    /// Helper: Cuenta archivos que inician con el prefijo dado
    fn count_descendants(&self, data_dir: &std::path::Path, prefix: &str) -> usize {
        use std::fs;
        
        let pattern = format!("{}.", prefix); // e.g., "3.1.7." para buscar "3.1.7.*"
        
        if let Ok(entries) = fs::read_dir(data_dir) {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    name.starts_with(&pattern) && name.ends_with(".md")
                })
                .count()
        } else {
            0
        }
    }

    /// L014: Detecta wikilinks con paths absolutos.
    /// Los wikilinks no deben usar prefijo de proyecto como "Proyecto OnlyCarNLD/Datos/".
    fn rule_wikilink_absolute_path(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use regex::Regex;
        lazy_static::lazy_static! {
            // Detecta paths absolutos en wikilinks
            static ref ABSOLUTE_PATH: Regex = Regex::new(r"\[\[Proyecto[^\]]*OnlyCarNLD/Datos/([^\]]+)\]\]").unwrap();
        }
        
        let mut issues = Vec::new();
        
        for (idx, line) in lines.iter().enumerate() {
            for cap in ABSOLUTE_PATH.captures_iter(line) {
                let full_match = cap.get(0).unwrap().as_str();
                issues.push(LintIssue {
                    code: "L014".to_string(),
                    message: format!("Wikilink con path absoluto: {}", &full_match[..full_match.len().min(50)]),
                    file: file_path.clone(),
                    line: Some(idx + 1),
                    severity: LintSeverity::Info,
                    fixable: false,
                });
            }
        }
        issues
    }

    // ═══════════════════════════════════════════════════════════════════════
    // L4.4: FIX AUTOMÁTICO
    // ═══════════════════════════════════════════════════════════════════════


    /// Corrige problemas fixables en un archivo.
    pub fn fix_file(&self, _file_path: &PathBuf, content: &str) -> Option<String> {
        let mut modified = false;
        let mut new_content = String::new();

        for line in content.lines() {
            // Fix L003: Trailing whitespace
            let trimmed = line.trim_end();
            if trimmed != line {
                modified = true;
            }
            new_content.push_str(trimmed);
            new_content.push('\n');
        }

        // Fix L004: Asegurar newline final
        if !content.ends_with('\n') {
            modified = true;
            // new_content ya termina con \n
        }

        // Remover newline extra al final si hay doble
        while new_content.ends_with("\n\n") {
            new_content.pop();
            modified = true;
        }

        if modified {
            Some(new_content)
        } else {
            None
        }
    }

    /// Verifica si una regla debe ejecutarse según el filtro --rule.
    fn should_run_rule(&self, rule_code: &str) -> bool {
        match &self.rule {
            Some(filter) => rule_code == filter,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_result_new() {
        let result = LintResult::new();
        assert!(result.is_clean());
    }

    #[test]
    fn test_lint_issue_error() {
        let issue = LintIssue::error("E001", "Missing title", PathBuf::from("test.md"));
        assert_eq!(issue.severity, LintSeverity::Error);
    }

    #[test]
    fn test_error_count() {
        let mut result = LintResult::new();
        result
            .issues
            .push(LintIssue::error("E001", "err", PathBuf::from("a.md")));
        result
            .issues
            .push(LintIssue::warning("W001", "warn", PathBuf::from("b.md")));

        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 1);
    }

    #[test]
    fn test_is_clean() {
        let mut result = LintResult::new();
        assert!(result.is_clean());

        result
            .issues
            .push(LintIssue::error("E001", "err", PathBuf::from("a.md")));
        assert!(!result.is_clean());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: LintCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    // F1.1: Priorizar cmd.path sobre cli.data_dir
    let default_dir = std::path::PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    for issue in &result.issues {
        let icon = match issue.severity {
            LintSeverity::Error => "❌",
            LintSeverity::Warning => "⚠️",
            LintSeverity::Info => "ℹ️",
            LintSeverity::Hint => "💡",
        };
        let line_info = issue.line.map(|l| format!(":{}", l)).unwrap_or_default();
        println!(
            "{} [{}] {}{}: {}",
            icon,
            issue.code,
            issue.file.display(),
            line_info,
            issue.message
        );
    }

    println!("\n📊 Lint Report:");
    println!("  📁 Archivos analizados: {}", result.files_checked);
    println!("  📝 Archivos con issues: {}", result.files_with_issues);
    println!("  ❌ Errores: {}", result.error_count());
    println!("  ⚠️  Warnings: {}", result.warning_count());
    println!("  🔧 Fixables: {}", result.fixable_count());

    if result.is_clean() {
        println!("\n✅ Sin problemas detectados");
    }

    Ok(())
}
