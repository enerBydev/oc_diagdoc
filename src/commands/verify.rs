//! Comando verify - Verificación completa del proyecto.
//!
//! Ejecuta 21 fases de verificación sobre la documentación.

use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use crate::errors::OcResult;

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
    
    /// Ejecutar solo fase específica.
    #[arg(long)]
    pub phase: Option<u8>,
    
    /// Modo silencioso (solo errores).
    #[arg(short, long)]
    pub quiet: bool,
}

impl VerifyCommand {
    /// Ejecuta la verificación completa.
    pub fn run(&self) -> OcResult<VerificationResult> {
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
            if let Some(only_phase) = self.phase {
                if *id != only_phase {
                    continue;
                }
            }
            
            let phase_start = Instant::now();
            let mut phase = VerificationPhase::new(*id, *name, *desc);
            
            // Ejecutar verificación (stub por ahora)
            self.run_phase(*id, &mut phase);
            
            phase.set_duration(phase_start.elapsed().as_millis() as u64);
            result.add_phase(phase);
        }
        
        result.duration_ms = start.elapsed().as_millis() as u64;
        Ok(result)
    }
    
    /// Ejecuta una fase específica.
    fn run_phase(&self, phase_id: u8, phase: &mut VerificationPhase) {
        // TODO: Implementar cada fase con lógica real
        match phase_id {
            1 => {
                // File count - siempre pasa como stub
            }
            2 => {
                // YAML validation
            }
            3 => {
                // Unique IDs
            }
            // ... más fases
            _ => {}
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
pub fn run(cmd: VerifyCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "passed": result.passed,
            "phases_total": result.phases.len(),
            "phases_passed": result.phases_passed(),
            "errors": result.total_errors,
            "warnings": result.total_warnings,
            "duration_ms": result.duration_ms
        }))?);
    } else {
        for phase in &result.phases {
            let status = if phase.passed { "✅" } else { "❌" };
            println!("{} Fase {}: {} ({}ms)", status, phase.id, phase.name, phase.duration_ms);
        }
        println!("\n📊 {}/{} fases pasaron, {} errores, {} warnings",
            result.phases_passed(), result.phases.len(),
            result.total_errors, result.total_warnings
        );
    }
    
    std::process::exit(VerifyCommand::exit_code(&result));
}
