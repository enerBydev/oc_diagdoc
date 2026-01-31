//! Comando help - Ayuda extendida.
//!
//! Muestra ayuda detallada y ejemplos de uso.

use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// HELP TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Sección de ayuda.
#[derive(Debug, Clone)]
pub struct HelpSection {
    pub title: String,
    pub content: String,
}

impl HelpSection {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

/// Resultado de ayuda.
#[derive(Debug, Clone)]
pub struct HelpResult {
    pub topic: String,
    pub sections: Vec<HelpSection>,
}

impl HelpResult {
    pub fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
            sections: Vec::new(),
        }
    }
    
    pub fn add_section(&mut self, section: HelpSection) {
        self.sections.push(section);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELP COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de ayuda extendida.
#[derive(Parser, Debug, Clone)]
#[command(name = "help", about = "Ayuda extendida")]
pub struct HelpCommand {
    /// Tema de ayuda.
    pub topic: Option<String>,
    
    /// Listar todos los temas.
    #[arg(short, long)]
    pub list: bool,
}

impl HelpCommand {
    pub fn run(&self) -> OcResult<HelpResult> {
        let topic = self.topic.as_deref().unwrap_or("general");
        let mut result = HelpResult::new(topic);
        
        result.add_section(HelpSection::new(
            "Descripción",
            "oc_diagdoc - Sistema de diagnóstico de documentación",
        ));
        result.add_section(HelpSection::new(
            "Uso",
            "oc_diagdoc <comando> [opciones]",
        ));
        
        Ok(result)
    }
    
    pub fn available_topics() -> Vec<&'static str> {
        vec![
            "general", "verify", "stats", "search", "deps", "tree",
            "lint", "health", "coverage", "export", "compress",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_section_new() {
        let section = HelpSection::new("Title", "Content");
        assert_eq!(section.title, "Title");
    }

    #[test]
    fn test_help_result_new() {
        let result = HelpResult::new("test");
        assert_eq!(result.topic, "test");
    }

    #[test]
    fn test_help_command_run() {
        let cmd = HelpCommand {
            topic: Some("verify".to_string()),
            list: false,
        };
        let result = cmd.run().unwrap();
        assert!(!result.sections.is_empty());
    }

    #[test]
    fn test_available_topics() {
        let topics = HelpCommand::available_topics();
        assert!(topics.contains(&"general"));
        assert!(topics.contains(&"verify"));
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: HelpCommand, _cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    if cmd.list {
        println!("📚 Temas de ayuda disponibles:\n");
        for topic in HelpCommand::available_topics() {
            println!("  • {}", topic);
        }
    } else {
        let result = cmd.run()?;
        println!("📖 Ayuda: {}\n", result.topic);
        for section in &result.sections {
            println!("## {}\n{}\n", section.title, section.content);
        }
    }
    
    Ok(())
}
