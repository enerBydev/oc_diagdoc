//! Comando module - Operaciones sobre módulos.
//!
//! Info, stats y operaciones sobre módulos específicos.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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
        Self {
            modules: Vec::new(),
        }
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

    // L25-L26: Flags avanzados
    /// Crear nuevo módulo.
    #[arg(long, short = 'c')]
    pub create: Option<String>,

    /// Mover documento a otro módulo.
    #[arg(long)]
    pub move_doc: Option<String>,

    /// Destino del move.
    #[arg(long)]
    pub to: Option<String>,
}

impl ModuleCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<ModuleResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashMap;

        let mut result = ModuleResult::new();

        // L25.2: Crear nuevo módulo
        if let Some(ref module_name) = self.create {
            return self.create_module(data_dir, module_name);
        }

        // L26.1: Mover documento a otro módulo
        if let (Some(ref doc), Some(ref dest)) = (&self.move_doc, &self.to) {
            return self.move_document(data_dir, doc, dest);
        }

        // L25.1: Listar módulos con stats
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let module_regex = Regex::new(r#"module:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let mut module_stats: HashMap<String, ModuleInfo> = HashMap::new();

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let module_name = module_regex
                    .captures(&content)
                    .map(|c| c[1].trim().to_string())
                    .unwrap_or_else(|| "sin_modulo".to_string());

                let word_count = content.split_whitespace().count();

                let entry = module_stats
                    .entry(module_name.clone())
                    .or_insert_with(|| ModuleInfo::new(&module_name, &module_name));

                entry.document_count += 1;
                entry.word_count += word_count;

                if let Some(ref filter) = self.module_id {
                    if &module_name == filter {
                        entry.children.push(file_path.display().to_string());
                    }
                }
            }
        }

        // Calcular health score
        for module in module_stats.values_mut() {
            let avg = module.avg_words();
            module.health_score = match avg {
                0..=50 => 40,
                51..=200 => 70,
                201..=500 => 90,
                _ => 100,
            };
        }

        // Filtrar por módulo_id si se especificó
        let modules: Vec<_> = if let Some(ref filter) = self.module_id {
            module_stats
                .into_values()
                .filter(|m| m.id.contains(filter))
                .collect()
        } else {
            module_stats.into_values().collect()
        };

        for module in modules {
            result.add_module(module);
        }

        Ok(result)
    }

    /// L25.2: Crear nuevo módulo.
    fn create_module(
        &self,
        data_dir: &std::path::Path,
        module_name: &str,
    ) -> OcResult<ModuleResult> {
        let module_dir = data_dir.join("docs").join("modulos").join(module_name);
        std::fs::create_dir_all(&module_dir)?;

        let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let readme = format!(
            r#"---
title: "Módulo {}"
document_id: "{}.0"
type: "module_root"
module: "{}"
status: "draft"
created: "{}"
last_updated: "{}"
---

# {}

## Descripción

[Descripción del módulo]

## Documentos

| ID | Título | Estado |
|----|--------|--------|
| - | - | - |
"#,
            module_name, module_name, module_name, now, now, module_name
        );

        std::fs::write(module_dir.join("README.md"), readme)?;

        let mut result = ModuleResult::new();
        let mut info = ModuleInfo::new(module_name, module_name);
        info.document_count = 1;
        result.add_module(info);

        eprintln!(
            "✅ Módulo '{}' creado en: {}",
            module_name,
            module_dir.display()
        );
        Ok(result)
    }

    /// L26.1: Mover documento a otro módulo.
    fn move_document(
        &self,
        data_dir: &std::path::Path,
        doc_id: &str,
        dest_module: &str,
    ) -> OcResult<ModuleResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let id_regex = Regex::new(r#"document_id:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let module_regex = Regex::new(r#"(module:\s*["']?)([^"'\n]+)(["']?)"#).unwrap();

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                if let Some(cap) = id_regex.captures(&content) {
                    if cap[1].trim() == doc_id {
                        // Actualizar módulo
                        let new_content = module_regex
                            .replace(&content, |caps: &regex::Captures| {
                                format!("{}{}{}", &caps[1], dest_module, &caps[3])
                            })
                            .to_string();

                        std::fs::write(file_path, &new_content)?;
                        eprintln!(
                            "✅ Documento '{}' movido a módulo '{}'",
                            doc_id, dest_module
                        );

                        let mut result = ModuleResult::new();
                        result.add_module(ModuleInfo::new(dest_module, dest_module));
                        return Ok(result);
                    }
                }
            }
        }

        Err(crate::errors::OcError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Documento '{}' no encontrado", doc_id),
        )))
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
pub fn run(cmd: ModuleCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    if cmd.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if cmd.list || result.modules.len() > 1 {
            println!("📦 Módulos ({}):\n", result.modules.len());
            for m in &result.modules {
                println!(
                    "  {} {} - {} docs, {} words, {}% health",
                    m.id, m.name, m.document_count, m.word_count, m.health_score
                );
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
