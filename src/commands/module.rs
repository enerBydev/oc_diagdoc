//! Comando module - Operaciones sobre módulos.
//!
//! Info, stats y operaciones sobre módulos específicos.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// MODULE INFO
// ═══════════════════════════════════════════════════════════════════════════

/// Información de un módulo.
#[derive(Debug, Clone, Serialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub document_count: usize,
    pub word_count: usize,
    pub health_score: u8,
    pub children: Vec<String>,
}

impl ModuleInfo {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            document_count: 0,
            word_count: 0,
            health_score: 100,
            children: Vec::new(),
        }
    }
    
    pub fn avg_words(&self) -> usize {
        if self.document_count == 0 {
            0
        } else {
            self.word_count / self.document_count
        }
    }
}

/// Resultado de operación sobre módulo.
#[derive(Debug, Clone, Serialize)]
pub struct ModuleResult {
    pub modules: Vec<ModuleInfo>,
}

impl ModuleResult {
    pub fn new() -> Self {
        Self { modules: Vec::new() }
    }
    
    pub fn add_module(&mut self, module: ModuleInfo) {
        self.modules.push(module);
    }
    
    pub fn total_documents(&self) -> usize {
        self.modules.iter().map(|m| m.document_count).sum()
    }
}

impl Default for ModuleResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODULE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de módulo.
#[derive(Parser, Debug, Clone)]
#[command(name = "module", about = "Operaciones sobre módulos")]
pub struct ModuleCommand {
    /// ID del módulo.
    pub module_id: Option<String>,
    
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Listar todos los módulos.
    #[arg(short, long)]
    pub list: bool,
    
    /// Output JSON.
    #[arg(long)]
    pub json: bool,
}

impl ModuleCommand {
    pub fn run(&self) -> OcResult<ModuleResult> {
        let result = ModuleResult::new();
        // TODO: Implementar operaciones reales
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info_new() {
        let info = ModuleInfo::new("1", "Plataforma");
        assert_eq!(info.id, "1");
        assert_eq!(info.health_score, 100);
    }

    #[test]
    fn test_avg_words() {
        let mut info = ModuleInfo::new("1", "Test");
        info.document_count = 10;
        info.word_count = 1000;
        
        assert_eq!(info.avg_words(), 100);
    }

    #[test]
    fn test_module_result() {
        let mut result = ModuleResult::new();
        let mut m1 = ModuleInfo::new("1", "A");
        m1.document_count = 5;
        
        let mut m2 = ModuleInfo::new("2", "B");
        m2.document_count = 10;
        
        result.add_module(m1);
        result.add_module(m2);
        
        assert_eq!(result.total_documents(), 15);
    }

    #[test]
    fn test_avg_words_empty() {
        let info = ModuleInfo::new("1", "Test");
        assert_eq!(info.avg_words(), 0);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ModuleCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let result = cmd.run()?;
    
    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if cmd.list || result.modules.len() > 1 {
            println!("📦 Módulos ({}):\n", result.modules.len());
            for m in &result.modules {
                println!("  {} {} - {} docs, {} words, {}% health",
                    m.id, m.name, m.document_count, m.word_count, m.health_score);
            }
        } else if let Some(m) = result.modules.first() {
            println!("📦 Módulo: {} {}", m.id, m.name);
            println!("📄 Documentos: {}", m.document_count);
            println!("📝 Palabras: {} (avg: {})", m.word_count, m.avg_words());
            println!("❤️  Salud: {}%", m.health_score);
        }
    }
    
    Ok(())
}
