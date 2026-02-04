//! Comando compress - Compilación de documentación.
//!
//! Compila toda la documentación en un solo archivo.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

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
            self.modules_included, self.documents_included, self.total_words, self.output_bytes
        )
    }
}

/// L6: Documento compilado para JSON/HTML output.
#[derive(Debug, Clone, Serialize)]
pub struct CompressedDoc {
    pub id: String,
    pub title: String,
    pub module: Option<String>,
    pub word_count: usize,
    pub content: String,
}

/// L6: Colección para JSON export.
#[derive(Debug, Clone, Serialize)]
pub struct CompressedCollection {
    pub generated: String,
    pub total_documents: usize,
    pub total_words: usize,
    pub modules: Vec<String>,
    pub documents: Vec<CompressedDoc>,
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

    /// Formato de salida (md/json/html).
    #[arg(short, long, default_value = "md")]
    pub format: String,

    /// Incluir solo módulos específicos.
    #[arg(short, long)]
    pub modules: Option<Vec<String>>,

    /// Excluir drafts.
    #[arg(long)]
    pub no_drafts: bool,

    // L6: Flags avanzados
    /// Dividir salida por módulo (crea múltiples archivos).
    #[arg(long)]
    pub split_by_module: bool,

    // B5: Nuevas flags de paridad con Python (doc_compiler.py)
    /// Generar versión PDF después de compilar (requiere pandoc).
    #[arg(long)]
    pub pdf: bool,

    /// Archivo de configuración YAML/JSON personalizado.
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,

    /// Modo estricto: falla en warnings.
    #[arg(long)]
    pub strict: bool,

    /// Omitir validaciones de calidad.
    #[arg(long)]
    pub skip_validation: bool,

    /// Incluir metadata YAML como bloque informativo.
    #[arg(long)]
    pub include_yaml: bool,

    /// P3-B4: Modo preview - mostrar output sin escribir archivo.
    #[arg(long)]
    pub preview: bool,
}

impl CompressCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<CompressResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        
        use std::collections::HashSet;

        let output = self
            .output
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("compiled.{}", self.format)));
        let mut result = CompressResult::new(output.clone());

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        // Regex para extraer metadata
        use crate::core::patterns::{RE_TITLE, RE_MODULE, RE_DRAFT};
        let title_regex = &*RE_TITLE;
        let module_regex = &*RE_MODULE;
        let draft_regex = &*RE_DRAFT;

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
                let file_id = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                // Filtrar por módulos si se especificó
                if let Some(ref module_filter) = self.modules {
                    let matches = module_filter.iter().any(|m| file_id.starts_with(m));
                    if !matches {
                        continue;
                    }
                }

                let title = title_regex
                    .captures(&content)
                    .map(|cap| cap[1].trim().to_string())
                    .unwrap_or_else(|| file_id.to_string());

                // Extraer módulo
                if let Some(cap) = module_regex.captures(&content) {
                    modules.insert(cap[1].to_string());
                }

                // Agregar a TOC
                let anchor = file_id.replace(['.', ' '], "-").to_lowercase();
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

        // Construir documento final según formato
        let final_content = format!("{}\n\n{}", toc, compiled_content);
        result.output_bytes = final_content.len();
        result.modules_included = modules.len();

        // L6: Escribir según formato
        match self.format.as_str() {
            "json" => {
                let collection = CompressedCollection {
                    generated: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    total_documents: result.documents_included,
                    total_words: result.total_words,
                    modules: modules.iter().cloned().collect(),
                    documents: Vec::new(), // Simplificado por ahora
                };
                let json = serde_json::to_string_pretty(&collection).unwrap_or_default();
                result.output_bytes = json.len();
                if self.preview {
                    println!("📋 Preview (primeras 50 líneas):\n{}", 
                             json.lines().take(50).collect::<Vec<_>>().join("\n"));
                } else {
                    std::fs::write(&output, &json)?;
                }
            }
            "html" => {
                let html = self.render_html(&toc, &compiled_content, result.documents_included);
                result.output_bytes = html.len();
                if self.preview {
                    println!("📋 Preview (primeras 50 líneas):\n{}", 
                             html.lines().take(50).collect::<Vec<_>>().join("\n"));
                } else {
                    std::fs::write(&output, &html)?;
                }
            }
            _ => {
                // Default: markdown
                if self.preview {
                    println!("📋 Preview (primeras 100 líneas):\n{}", 
                             final_content.lines().take(100).collect::<Vec<_>>().join("\n"));
                    println!("\n... ({} líneas más)", final_content.lines().count().saturating_sub(100));
                } else {
                    std::fs::write(&output, &final_content)?;
                }
            }
        }

        // B5: Generar PDF si se solicitó
        if self.pdf {
            let pdf_output = output.with_extension("pdf");
            let status = std::process::Command::new("pandoc")
                .args([
                    output.to_str().unwrap_or_default(),
                    "-o",
                    pdf_output.to_str().unwrap_or_default(),
                    "--pdf-engine=pdflatex",
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    eprintln!("📄 PDF generado: {}", pdf_output.display());
                }
                _ => {
                    eprintln!("⚠️ Error generando PDF (¿pandoc instalado?)");
                }
            }
        }

        Ok(result)
    }

    /// L6.3: Genera HTML con wrapper y CSS básico.
    fn render_html(&self, toc: &str, content: &str, doc_count: usize) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentación Compilada</title>
    <style>
        :root {{ --primary: #2563eb; --bg: #f8fafc; --text: #1e293b; }}
        body {{ font-family: system-ui, sans-serif; background: var(--bg); color: var(--text); max-width: 900px; margin: 0 auto; padding: 2rem; line-height: 1.6; }}
        h1, h2, h3 {{ color: var(--primary); }}
        pre {{ background: #1e293b; color: #e2e8f0; padding: 1rem; border-radius: 8px; overflow-x: auto; }}
        code {{ background: #e2e8f0; padding: 0.2rem 0.4rem; border-radius: 4px; }}
        a {{ color: var(--primary); }}
        .toc {{ background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 1.5rem; margin-bottom: 2rem; }}
        .stats {{ color: #64748b; font-size: 0.875rem; margin-bottom: 2rem; }}
    </style>
</head>
<body>
    <h1>📚 Documentación Compilada</h1>
    <p class="stats">Generado: {} | {} documentos</p>
    <div class="toc">
        {}
    </div>
    <div class="content">
        {}
    </div>
</body>
</html>"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M"),
            doc_count,
            toc.replace("\n", "<br>"),
            content.replace("\n", "<br>")
        )
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
            split_by_module: false,
            pdf: false,
            config: None,
            strict: false,
            skip_validation: false,
            include_yaml: false,
            preview: false,
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
            split_by_module: false,
            pdf: false,
            config: None,
            strict: false,
            skip_validation: false,
            include_yaml: false,
            preview: false,
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
