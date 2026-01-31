//! Comando compress - Compilación de documentación.
//!
//! Compila toda la documentación en un solo archivo.

use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// COMPRESS TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de compresión.
#[derive(Debug, Clone, Serialize)]
pub struct CompressResult {
    pub output_path: PathBuf,
    pub modules_included: usize,
    pub documents_included: usize,
    pub total_words: usize,
    pub output_bytes: usize,
}

impl CompressResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            output_path: path,
            modules_included: 0,
            documents_included: 0,
            total_words: 0,
            output_bytes: 0,
        }
    }
    
    pub fn summary(&self) -> String {
        format!(
            "{} módulos, {} docs, {} palabras → {} bytes",
            self.modules_included,
            self.documents_included,
            self.total_words,
            self.output_bytes
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPRESS COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de compresión/compilación.
#[derive(Parser, Debug, Clone)]
#[command(name = "compress", about = "Compilar documentación")]
pub struct CompressCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Archivo de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    
    /// Formato de salida (md/pdf/html).
    #[arg(short, long, default_value = "md")]
    pub format: String,
    
    /// Incluir solo módulos específicos.
    #[arg(short, long)]
    pub modules: Option<Vec<String>>,
    
    /// Excluir drafts.
    #[arg(long)]
    pub no_drafts: bool,
}

impl CompressCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<CompressResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashSet;
        
        let output = self.output.clone().unwrap_or_else(|| {
            PathBuf::from(format!("compiled.{}", self.format))
        });
        let mut result = CompressResult::new(output.clone());
        
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        
        // Regex para extraer metadata
        let title_regex = Regex::new(r#"title:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let module_regex = Regex::new(r#"module:\s*["']?([^"'\n]+)["']?"#).unwrap();
        let draft_regex = Regex::new(r#"draft:\s*true"#).unwrap();
        
        // Ordenar archivos por nombre (que incluye la numeración)
        let mut sorted_files: Vec<_> = files.clone();
        sorted_files.sort();
        
        let mut compiled_content = String::new();
        let mut toc = String::from("# 📑 Tabla de Contenidos\n\n");
        let mut modules: HashSet<String> = HashSet::new();
        
        // Header del documento compilado
        compiled_content.push_str(&format!(
            "---\ntitle: Documentación Compilada\ngenerated: {}\n---\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));
        
        for file_path in &sorted_files {
            if let Ok(content) = read_file_content(file_path) {
                // Filtrar drafts si se pidió
                if self.no_drafts && draft_regex.is_match(&content) {
                    continue;
                }
                
                // Extraer ID y título
                let file_id = file_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                
                // Filtrar por módulos si se especificó
                if let Some(ref module_filter) = self.modules {
                    let matches = module_filter.iter().any(|m| file_id.starts_with(m));
                    if !matches {
                        continue;
                    }
                }
                
                let title = title_regex.captures(&content)
                    .map(|cap| cap[1].trim().to_string())
                    .unwrap_or_else(|| file_id.to_string());
                
                // Extraer módulo
                if let Some(cap) = module_regex.captures(&content) {
                    modules.insert(cap[1].to_string());
                }
                
                // Agregar a TOC
                let anchor = file_id.replace('.', "-").replace(' ', "-").to_lowercase();
                toc.push_str(&format!("- [{}](#{})\n", title, anchor));
                
                // Agregar separador y contenido
                compiled_content.push_str(&format!("\n---\n\n## {} {{{}}}\n\n", title, file_id));
                
                // Remover frontmatter del contenido antes de agregarlo
                let content_body = if content.starts_with("---") {
                    if let Some(end) = content[3..].find("---") {
                        content[end + 6..].trim()
                    } else {
                        content.as_str()
                    }
                } else {
                    content.as_str()
                };
                
                compiled_content.push_str(content_body);
                compiled_content.push_str("\n\n");
                
                result.documents_included += 1;
                result.total_words += content.split_whitespace().count();
            }
        }
        
        // Construir documento final
        let final_content = format!("{}\n\n{}", toc, compiled_content);
        result.output_bytes = final_content.len();
        result.modules_included = modules.len();
        
        // Escribir archivo si es markdown
        if self.format == "md" {
            std::fs::write(&output, &final_content)?;
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_result_new() {
        let result = CompressResult::new(PathBuf::from("out.md"));
        assert_eq!(result.modules_included, 0);
    }

    #[test]
    fn test_compress_result_summary() {
        let mut result = CompressResult::new(PathBuf::from("out.md"));
        result.modules_included = 5;
        result.documents_included = 100;
        result.total_words = 50000;
        result.output_bytes = 250000;
        
        let summary = result.summary();
        assert!(summary.contains("5 módulos"));
        assert!(summary.contains("100 docs"));
    }

    #[test]
    fn test_compress_command_run() {
        let temp_dir = std::env::temp_dir();
        let cmd = CompressCommand {
            path: None,
            output: Some(PathBuf::from("/tmp/test_compress.md")),
            format: "md".to_string(),
            modules: None,
            no_drafts: false,
        };
        let result = cmd.run(&temp_dir).unwrap();
        assert_eq!(result.output_path, PathBuf::from("/tmp/test_compress.md"));
    }

    #[test]
    fn test_compress_default_output() {
        let temp_dir = std::env::temp_dir();
        let cmd = CompressCommand {
            path: None,
            output: None,
            format: "pdf".to_string(),
            modules: None,
            no_drafts: false,
        };
        let result = cmd.run(&temp_dir).unwrap();
        assert!(result.output_path.to_str().unwrap().ends_with(".pdf"));
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: CompressCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;
    
    println!("📦 Compilando documentación...");
    println!("📁 Salida: {}", result.output_path.display());
    println!("📊 {}", result.summary());
    
    Ok(())
}
