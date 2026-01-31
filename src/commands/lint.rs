//! Comando lint - Análisis estático de documentación.
//!
//! Detecta problemas de estilo y estructura.

use std::path::PathBuf;
use clap::Parser;
use crate::errors::OcResult;

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
        self.issues.iter().filter(|i| i.severity == LintSeverity::Error).count()
    }
    
    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| i.severity == LintSeverity::Warning).count()
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
    
    /// Corregir automáticamente.
    #[arg(long)]
    pub fix: bool,
    
    /// Solo errores.
    #[arg(long)]
    pub errors_only: bool,
    
    /// Output formato JSON.
    #[arg(long)]
    pub json: bool,
}

impl LintCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<LintResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use std::collections::HashSet;
        
        let mut result = LintResult::new();
        
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        
        result.files_checked = files.len();
        let mut files_with_issues_set: HashSet<PathBuf> = HashSet::new();
        
        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let issues = self.lint_file(file_path, &content);
                
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
        Ok(result)
    }
    
    /// Aplica todas las reglas a un archivo.
    fn lint_file(&self, file_path: &PathBuf, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Regla 1: Archivo debe tener frontmatter YAML
        issues.extend(self.rule_frontmatter(file_path, content));
        
        // Regla 2: Headers deben ser jerárquicos
        issues.extend(self.rule_header_hierarchy(file_path, &lines));
        
        // Regla 3: No trailing whitespace
        issues.extend(self.rule_trailing_whitespace(file_path, &lines));
        
        // Regla 4: Archivo termina con newline
        issues.extend(self.rule_final_newline(file_path, content));
        
        // Regla 5: No líneas > 300 caracteres (muy largas)
        issues.extend(self.rule_line_length(file_path, &lines));
        
        // Regla 6: Code blocks deben tener lenguaje
        issues.extend(self.rule_code_block_language(file_path, &lines));
        
        // Regla 7: Headers no duplicados
        issues.extend(self.rule_duplicate_headers(file_path, &lines));
        
        // Regla 8: Frontmatter fields obligatorios
        issues.extend(self.rule_required_fields(file_path, content));
        
        issues
    }
    
    /// Regla: Archivo debe tener frontmatter YAML.
    fn rule_frontmatter(&self, file_path: &PathBuf, content: &str) -> Vec<LintIssue> {
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
        
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with('#') && !line.starts_with("```") {
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
        let mut issues = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            if line.len() > 300 {
                issues.push(LintIssue {
                    code: "L005".to_string(),
                    message: format!("Línea muy larga ({} chars)", line.len()),
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
    fn rule_code_block_language(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            if line.trim() == "```" {
                issues.push(LintIssue {
                    code: "L006".to_string(),
                    message: "Code block sin lenguaje especificado".to_string(),
                    file: file_path.clone(),
                    line: Some(idx + 1),
                    severity: LintSeverity::Hint,
                    fixable: false,
                });
            }
        }
        issues
    }
    
    /// Regla: Headers no duplicados.
    fn rule_duplicate_headers(&self, file_path: &PathBuf, lines: &[&str]) -> Vec<LintIssue> {
        use std::collections::HashMap;
        let mut issues = Vec::new();
        let mut seen: HashMap<String, usize> = HashMap::new();
        
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with('#') && !line.starts_with("```") {
                let header = line.trim_start_matches('#').trim().to_lowercase();
                if let Some(prev_line) = seen.get(&header) {
                    issues.push(LintIssue {
                        code: "L007".to_string(),
                        message: format!("Header duplicado (primera aparición línea {})", prev_line),
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
                        message: format!("Campo obligatorio faltante: {}", field.trim_end_matches(':')),
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
        result.issues.push(LintIssue::error("E001", "err", PathBuf::from("a.md")));
        result.issues.push(LintIssue::warning("W001", "warn", PathBuf::from("b.md")));
        
        assert_eq!(result.error_count(), 1);
        assert_eq!(result.warning_count(), 1);
    }

    #[test]
    fn test_is_clean() {
        let mut result = LintResult::new();
        assert!(result.is_clean());
        
        result.issues.push(LintIssue::error("E001", "err", PathBuf::from("a.md")));
        assert!(!result.is_clean());
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: LintCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;
    
    for issue in &result.issues {
        let icon = match issue.severity {
            LintSeverity::Error => "❌",
            LintSeverity::Warning => "⚠️",
            LintSeverity::Info => "ℹ️",
            LintSeverity::Hint => "💡",
        };
        let line_info = issue.line.map(|l| format!(":{}", l)).unwrap_or_default();
        println!("{} [{}] {}{}: {}", icon, issue.code, issue.file.display(), line_info, issue.message);
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
