//! Comando audit - Auditoría completa del proyecto.
//!
//! Genera informe detallado de problemas y recomendaciones.

use crate::errors::OcResult;
use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// AUDIT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Categoría de hallazgo de auditoría.
#[derive(Debug, Clone, Serialize)]
pub enum AuditCategory {
    Structure,
    Content,
    Metadata,
    Links,
    Performance,
    Security,
}

/// Un hallazgo de auditoría.
#[derive(Debug, Clone, Serialize)]
pub struct AuditFinding {
    pub category: AuditCategory,
    pub severity: u8, // 1-5
    pub title: String,
    pub description: String,
    pub recommendation: String,
    pub affected_files: Vec<PathBuf>,
}

/// Resultado de auditoría.
#[derive(Debug, Clone, Serialize)]
pub struct AuditResult {
    pub timestamp: String,
    pub findings: Vec<AuditFinding>,
    pub total_files: usize,
    pub score: u8,
}

impl AuditResult {
    pub fn new() -> Self {
        let now: DateTime<Utc> = std::time::SystemTime::now().into();
        Self {
            timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            findings: Vec::new(),
            total_files: 0,
            score: 100,
        }
    }

    pub fn add_finding(&mut self, finding: AuditFinding) {
        // Penalizar score basado en severidad
        let penalty = finding.severity * 2;
        self.score = self.score.saturating_sub(penalty);
        self.findings.push(finding);
    }

    pub fn critical_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity >= 4).count()
    }

    pub fn by_category(&self, cat: &AuditCategory) -> Vec<&AuditFinding> {
        self.findings
            .iter()
            .filter(|f| std::mem::discriminant(&f.category) == std::mem::discriminant(cat))
            .collect()
    }
}

impl Default for AuditResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AUDIT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de auditoría.
#[derive(Parser, Debug, Clone)]
#[command(name = "audit", about = "Auditoría completa")]
pub struct AuditCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Output JSON.
    #[arg(long)]
    pub json: bool,

    /// Incluir recomendaciones priorizadas.
    #[arg(long)]
    pub recommendations: bool,

    // L10: Flags avanzados
    /// Exportar a archivo JSON.
    #[arg(long, value_name = "FILE")]
    pub export: Option<PathBuf>,

    /// Mostrar detalles completos de cada finding.
    #[arg(long)]
    pub verbose: bool,
}

impl AuditResult {
    // L10: Métodos avanzados

    /// L10.1: Ordena findings por severidad (mayor primero).
    pub fn sorted_by_severity(&self) -> Vec<&AuditFinding> {
        let mut findings: Vec<_> = self.findings.iter().collect();
        findings.sort_by(|a, b| b.severity.cmp(&a.severity));
        findings
    }

    /// L10.2: Estima esfuerzo para resolver un finding (minutos).
    pub fn estimate_effort(finding: &AuditFinding) -> u32 {
        match finding.severity {
            5 => 60, // Crítico: ~1 hora
            4 => 30, // Alto: ~30 min
            3 => 15, // Medio: ~15 min
            2 => 5,  // Bajo: ~5 min
            _ => 2,  // Muy bajo: ~2 min
        }
    }

    /// L10.3: Genera JSON formateado para export.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// L10.1: Genera lista de recomendaciones priorizadas.
    pub fn prioritized_recommendations(&self) -> Vec<String> {
        self.sorted_by_severity()
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let effort = Self::estimate_effort(f);
                format!(
                    "{}. [S{}] {} (~{}min): {}",
                    i + 1,
                    f.severity,
                    f.title,
                    effort,
                    f.recommendation
                )
            })
            .collect()
    }
}

impl AuditCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<AuditResult> {
        use crate::commands::links::{LinkStatus, LinksCommand};
        use crate::commands::lint::{LintCommand, LintSeverity};
        use crate::core::files::{get_all_md_files, ScanOptions};

        let mut result = AuditResult::new();

        // Contar archivos
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        result.total_files = files.len();

        // 1. Ejecutar análisis de links
        let links_cmd = LinksCommand {
            path: None,
            broken_only: false,
            include_external: false,
            fix: false,
            find_refs: None,
            rename: None,
            backup: false,
        };
        if let Ok(links_result) = links_cmd.run(data_dir) {
            // Finding: Enlaces rotos
            if links_result.total_broken > 0 {
                result.add_finding(AuditFinding {
                    category: AuditCategory::Links,
                    severity: 4,
                    title: format!("{} enlaces rotos detectados", links_result.total_broken),
                    description: "Existen enlaces que apuntan a documentos inexistentes."
                        .to_string(),
                    recommendation:
                        "Ejecutar `oc_diagdoc links --broken-only` para listar y corregir."
                            .to_string(),
                    affected_files: links_result
                        .broken_links()
                        .iter()
                        .take(10)
                        .map(|l| l.source.clone())
                        .collect(),
                });
            }

            // Finding: Enlaces no-estándar
            if links_result.total_nonstandard > 0 {
                result.add_finding(AuditFinding {
                    category: AuditCategory::Links,
                    severity: 3,
                    title: format!(
                        "{} enlaces con path completo (no-estándar)",
                        links_result.total_nonstandard
                    ),
                    description:
                        "Los enlaces wiki deberían usar solo el nombre del archivo, sin path."
                            .to_string(),
                    recommendation: "Cambiar [[Proyecto/Datos/doc]] a [[doc]]".to_string(),
                    affected_files: links_result
                        .links
                        .iter()
                        .filter(|l| l.status == LinkStatus::NonStandard)
                        .take(10)
                        .map(|l| l.source.clone())
                        .collect(),
                });
            }
        }

        // 2. Ejecutar análisis de lint
        let lint_cmd = LintCommand {
            path: None,
            fix: false,
            dry_run: false,
            errors_only: false,
            json: false,
            rule: None,
            summary: false,
            show_fixes: false,
            explain: None,  // RFC-03
        };
        if let Ok(lint_result) = lint_cmd.run(data_dir) {
            // Finding: Errores de lint
            if lint_result.error_count() > 0 {
                result.add_finding(AuditFinding {
                    category: AuditCategory::Content,
                    severity: 4,
                    title: format!(
                        "{} errores de formato detectados",
                        lint_result.error_count()
                    ),
                    description: "Archivos con problemas de estructura o campos faltantes."
                        .to_string(),
                    recommendation: "Ejecutar `oc_diagdoc lint --errors-only` para detalles."
                        .to_string(),
                    affected_files: lint_result
                        .issues
                        .iter()
                        .filter(|i| i.severity == LintSeverity::Error)
                        .take(10)
                        .map(|i| i.file.clone())
                        .collect(),
                });
            }

            // Finding: Warnings de lint
            if lint_result.warning_count() > 0 {
                result.add_finding(AuditFinding {
                    category: AuditCategory::Content,
                    severity: 2,
                    title: format!("{} warnings de formato", lint_result.warning_count()),
                    description: "Problemas menores de formato o estilo.".to_string(),
                    recommendation: "Ejecutar `oc_diagdoc lint` para ver todos los warnings."
                        .to_string(),
                    affected_files: vec![],
                });
            }
        }

        // 3. Análisis de cobertura rápido
        let mut low_content_files = Vec::new();
        for file_path in &files {
            if let Ok(content) = crate::core::files::read_file_content(file_path) {
                let word_count = content.split_whitespace().count();
                if word_count < 100 {
                    low_content_files.push(file_path.clone());
                }
            }
        }

        if low_content_files.len() > 10 {
            result.add_finding(AuditFinding {
                category: AuditCategory::Content,
                severity: 3,
                title: format!(
                    "{} documentos con bajo contenido (<100 palabras)",
                    low_content_files.len()
                ),
                description: "Documentos que parecen ser placeholders o están incompletos."
                    .to_string(),
                recommendation: "Revisar y completar el contenido de estos documentos.".to_string(),
                affected_files: low_content_files.into_iter().take(10).collect(),
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_result_new() {
        let result = AuditResult::new();
        assert_eq!(result.score, 100);
    }

    #[test]
    fn test_add_finding() {
        let mut result = AuditResult::new();
        result.add_finding(AuditFinding {
            category: AuditCategory::Structure,
            severity: 3,
            title: "Missing index".to_string(),
            description: "desc".to_string(),
            recommendation: "rec".to_string(),
            affected_files: vec![],
        });

        assert_eq!(result.score, 94); // 100 - 3*2
    }

    #[test]
    fn test_critical_count() {
        let mut result = AuditResult::new();
        result.add_finding(AuditFinding {
            category: AuditCategory::Security,
            severity: 5,
            title: "Critical".to_string(),
            description: "d".to_string(),
            recommendation: "r".to_string(),
            affected_files: vec![],
        });

        assert_eq!(result.critical_count(), 1);
    }

    #[test]
    fn test_score_saturation() {
        let mut result = AuditResult::new();
        for _ in 0..100 {
            result.add_finding(AuditFinding {
                category: AuditCategory::Content,
                severity: 5,
                title: "t".to_string(),
                description: "d".to_string(),
                recommendation: "r".to_string(),
                affected_files: vec![],
            });
        }

        assert_eq!(result.score, 0); // No underflow
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: AuditCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    // F1.3: Priorizar cmd.path sobre cli.data_dir
    let default_dir = std::path::PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🔍 Auditoría - {}", result.timestamp);
        println!("📁 Archivos: {}", result.total_files);
        println!("📊 Score: {}/100", result.score);
        println!(
            "⚠️  {} hallazgos ({} críticos)",
            result.findings.len(),
            result.critical_count()
        );

        for f in &result.findings {
            let icon = match f.severity {
                5 => "🔴",
                4 => "🟠",
                3 => "🟡",
                2 => "🟢",
                _ => "⚪",
            };
            println!("\n{} [Sev {}] {}", icon, f.severity, f.title);
            if cmd.recommendations {
                println!("  💡 {}", f.recommendation);
            }
        }
    }

    // AN-02 FIX: Implementar escritura a archivo cuando --export está presente
    if let Some(export_path) = &cmd.export {
        let json = serde_json::to_string_pretty(&result)?;
        std::fs::write(export_path, &json)?;
        println!("📄 Exportado a: {}", export_path.display());
    }

    Ok(())
}
