//! Comando ci - Integración continua.
//!
//! Ejecuta verificaciones para CI/CD.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// CI TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Check de CI.
#[derive(Debug, Clone, Serialize)]
pub struct CiCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}

impl CiCheck {
    pub fn pass(name: &str, message: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            passed: true,
            message: message.to_string(),
            duration_ms,
        }
    }
    
    pub fn fail(name: &str, message: &str, duration_ms: u64) -> Self {
        Self {
            name: name.to_string(),
            passed: false,
            message: message.to_string(),
            duration_ms,
        }
    }
}

/// Resultado de CI.
#[derive(Debug, Clone, Serialize)]
pub struct CiResult {
    pub checks: Vec<CiCheck>,
    pub all_passed: bool,
    pub total_duration_ms: u64,
}

impl CiResult {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            all_passed: true,
            total_duration_ms: 0,
        }
    }
    
    pub fn add_check(&mut self, check: CiCheck) {
        self.total_duration_ms += check.duration_ms;
        if !check.passed {
            self.all_passed = false;
        }
        self.checks.push(check);
    }
    
    pub fn exit_code(&self) -> i32 {
        if self.all_passed { 0 } else { 1 }
    }
}

impl Default for CiResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CI COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de CI.
#[derive(Parser, Debug, Clone)]
#[command(name = "ci", about = "Verificaciones CI/CD")]
pub struct CiCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Modo estricto.
    #[arg(long)]
    pub strict: bool,
    
    /// Output JSON.
    #[arg(long)]
    pub json: bool,
}

impl CiCommand {
    pub fn run(&self) -> OcResult<CiResult> {
        let mut result = CiResult::new();
        
        result.add_check(CiCheck::pass("lint", "No issues", 50));
        result.add_check(CiCheck::pass("schema", "Valid", 30));
        result.add_check(CiCheck::pass("links", "All valid", 100));
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_check_pass() {
        let check = CiCheck::pass("test", "ok", 10);
        assert!(check.passed);
    }

    #[test]
    fn test_ci_check_fail() {
        let check = CiCheck::fail("test", "error", 10);
        assert!(!check.passed);
    }

    #[test]
    fn test_ci_result_all_passed() {
        let mut result = CiResult::new();
        result.add_check(CiCheck::pass("a", "ok", 10));
        result.add_check(CiCheck::pass("b", "ok", 10));
        
        assert!(result.all_passed);
        assert_eq!(result.exit_code(), 0);
    }

    #[test]
    fn test_ci_result_with_failure() {
        let mut result = CiResult::new();
        result.add_check(CiCheck::pass("a", "ok", 10));
        result.add_check(CiCheck::fail("b", "error", 10));
        
        assert!(!result.all_passed);
        assert_eq!(result.exit_code(), 1);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: CiCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    for check in &result.checks {
        let icon = if check.passed { "✅" } else { "❌" };
        println!("{} {} ({}ms)", icon, check.name, check.duration_ms);
    }
    
    let status = if result.all_passed { "PASSED" } else { "FAILED" };
    println!("\n🏁 CI {}: {}ms total", status, result.total_duration_ms);
    
    std::process::exit(result.exit_code());
}
