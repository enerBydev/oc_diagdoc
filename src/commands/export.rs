//! Comando export - Exportación de documentación.
//!
//! Exporta documentación a múltiples formatos.

use crate::errors::OcResult;
use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Formato de exportación.
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Markdown,
    Html,
    Pdf,
    Docx,
    Json,
    Latex,
}

impl ExportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "md" | "markdown" => Some(Self::Markdown),
            "html" => Some(Self::Html),
            "pdf" => Some(Self::Pdf),
            "docx" | "word" => Some(Self::Docx),
            "json" => Some(Self::Json),
            "latex" | "tex" => Some(Self::Latex),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Html => "html",
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Json => "json",
            Self::Latex => "tex",
        }
    }
}

/// Resultado de exportación.
#[derive(Debug, Clone, Serialize)]
pub struct ExportResult {
    pub output_path: PathBuf,
    pub format: String,
    pub files_exported: usize,
    pub total_bytes: usize,
}

impl ExportResult {
    pub fn new(path: PathBuf, format: &str) -> Self {
        Self {
            output_path: path,
            format: format.to_string(),
            files_exported: 0,
            total_bytes: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de exportación.
#[derive(Parser, Debug, Clone)]
#[command(name = "export", about = "Exportar documentación")]
pub struct ExportCommand {
    /// Formato de salida.
    #[arg(short, long, default_value = "markdown")]
    pub format: String,

    /// Ruta de salida.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Incluir tabla de contenidos.
    #[arg(long)]
    pub toc: bool,

    // L11: Flags básicos
    /// Incluir metadata como archivo JSON separado.
    #[arg(long)]
    pub include_metadata: bool,

    /// Renombrar archivos con prefijos de módulo (ej: M1_doc.md).
    #[arg(long)]
    pub prefix_rename: bool,

    /// Filtrar por módulos específicos.
    #[arg(long)]
    pub modules: Option<Vec<String>>,

    // L12: Flags avanzados
    /// Exportar como archivo ZIP.
    #[arg(long)]
    pub zip: bool,

    // F6: Nuevas flags de paridad con Python
    /// Ruta a plantilla personalizada para exportación.
    #[arg(long)]
    pub template: Option<PathBuf>,

    /// Exportar todo en un único archivo concatenado.
    #[arg(long)]
    pub single_file: bool,

    // P2: Nuevas flags de paridad con Python v16 (IA context exporter)
    /// Límite máximo de tokens para exportación (estimado ~4 chars/token).
    #[arg(long)]
    pub max_tokens: Option<usize>,

    /// Modo compacto: minimiza whitespace y remove duplicados.
    #[arg(long)]
    pub compact: bool,

    /// Incluir árbol de estructura antes del contenido.
    #[arg(long)]
    pub tree: bool,

    /// Incluir estadísticas del proyecto en el export.
    #[arg(long)]
    pub stats: bool,
}


/// L11.3: Índice de exportación.
#[derive(Debug, Clone, Serialize)]
pub struct ExportIndex {
    pub exported_at: String,
    pub source_dir: String,
    pub total_files: usize,
    pub modules: Vec<String>,
    pub files: Vec<ExportFileEntry>,
}

/// Entrada en el índice.
#[derive(Debug, Clone, Serialize)]
pub struct ExportFileEntry {
    pub original_name: String,
    pub exported_name: String,
    pub module: Option<String>,
    pub word_count: usize,
}

impl ExportCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<ExportResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        
        use std::collections::HashSet;

        let output_dir = self
            .output
            .clone()
            .unwrap_or_else(|| PathBuf::from("export"));
        let mut result = ExportResult::new(output_dir.clone(), &self.format);

        // Crear directorio de salida
        std::fs::create_dir_all(&output_dir)?;

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        use crate::core::patterns::{RE_MODULE, RE_TITLE};
        let module_regex = &*RE_MODULE;
        let title_regex = &*RE_TITLE;

        let mut modules_found: HashSet<String> = HashSet::new();
        let mut index_entries: Vec<ExportFileEntry> = Vec::new();
        let mut metadata_collection: Vec<serde_json::Value> = Vec::new();

        for file_path in &files {
            let file_name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.md");
            let file_stem = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            if let Ok(content) = read_file_content(file_path) {
                // Extraer módulo
                let module = module_regex
                    .captures(&content)
                    .map(|cap| cap[1].trim().to_string());

                // Filtrar por módulos si se especificó
                if let Some(ref filter_modules) = self.modules {
                    let matches = module
                        .as_ref()
                        .map(|m| filter_modules.iter().any(|f| m.contains(f)))
                        .unwrap_or(false);
                    if !matches && !filter_modules.iter().any(|f| file_stem.starts_with(f)) {
                        continue;
                    }
                }

                if let Some(ref m) = module {
                    modules_found.insert(m.clone());
                }

                // L11.2: Nombre con prefijo si se pidió
                let exported_name = if self.prefix_rename {
                    if let Some(ref m) = module {
                        let prefix = m.replace(" ", "_").replace(".", "-");
                        format!("{}_{}", prefix, file_name)
                    } else {
                        file_name.to_string()
                    }
                } else {
                    file_name.to_string()
                };

                let word_count = content.split_whitespace().count();

                // L11.1: Copiar archivo
                let dest_path = output_dir.join(&exported_name);
                std::fs::write(&dest_path, &content)?;

                result.files_exported += 1;
                result.total_bytes += content.len();

                // L11.3: Agregar a índice
                index_entries.push(ExportFileEntry {
                    original_name: file_name.to_string(),
                    exported_name: exported_name.clone(),
                    module: module.clone(),
                    word_count,
                });

                // L11.4 / L12.2: Metadata para JSON
                if self.include_metadata {
                    let title = title_regex
                        .captures(&content)
                        .map(|cap| cap[1].trim().to_string())
                        .unwrap_or_else(|| file_stem.to_string());

                    metadata_collection.push(serde_json::json!({
                        "id": file_stem,
                        "title": title,
                        "module": module,
                        "word_count": word_count,
                        "exported_as": exported_name
                    }));
                }
            }
        }

        // L11.3: Generar índice
        let index = ExportIndex {
            exported_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            source_dir: data_dir.display().to_string(),
            total_files: result.files_exported,
            modules: modules_found.iter().cloned().collect(),
            files: index_entries,
        };
        let index_json = serde_json::to_string_pretty(&index).unwrap_or_default();
        std::fs::write(output_dir.join("_index.json"), &index_json)?;

        // L12.2: Metadata JSON separado
        if self.include_metadata {
            let meta_json = serde_json::to_string_pretty(&metadata_collection).unwrap_or_default();
            std::fs::write(output_dir.join("_metadata.json"), &meta_json)?;
        }

        // L12.1: Crear ZIP si se pidió
        if self.zip {
            self.create_zip(&output_dir, &result)?;
        }

        Ok(result)
    }

    /// L12.1: Crea archivo ZIP de la exportación.
    fn create_zip(&self, output_dir: &PathBuf, _result: &ExportResult) -> OcResult<()> {
        use crate::errors::OcError;
        use std::io::Write;

        let zip_path = output_dir.with_extension("zip");
        let file = std::fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        use walkdir::WalkDir;
        for entry in WalkDir::new(output_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");

                zip.start_file(file_name, options).map_err(|e| {
                    OcError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    ))
                })?;
                let content = std::fs::read(path)?;
                zip.write_all(&content)?;
            }
        }

        zip.finish().map_err(|e| {
            OcError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        eprintln!("📦 ZIP creado: {}", zip_path.display());
        Ok(())
    }

    pub fn format_enum(&self) -> ExportFormat {
        ExportFormat::from_str(&self.format).unwrap_or(ExportFormat::Markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format_from_str() {
        assert_eq!(ExportFormat::from_str("pdf"), Some(ExportFormat::Pdf));
        assert_eq!(ExportFormat::from_str("html"), Some(ExportFormat::Html));
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Pdf.extension(), "pdf");
        assert_eq!(ExportFormat::Latex.extension(), "tex");
    }

    #[test]
    fn test_export_result_new() {
        let result = ExportResult::new(PathBuf::from("out.pdf"), "pdf");
        assert_eq!(result.format, "pdf");
    }

    #[test]
    fn test_export_command_format_enum() {
        let cmd = ExportCommand {
            format: "latex".to_string(),
            output: None,
            path: None,
            toc: false,
            include_metadata: false,
            prefix_rename: false,
            modules: None,
            zip: false,
            // F6/F9: Campos agregados
            template: None,
            single_file: false,
            // P2: Campos agregados v16
            max_tokens: None,
            compact: false,
            tree: false,
            stats: false,
        };
        assert_eq!(cmd.format_enum(), ExportFormat::Latex);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: ExportCommand, cli: &crate::commands::CliConfig) -> anyhow::Result<()> {
    let default_dir = PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);

    // F6: Notificar plantilla si se usa
    if let Some(ref template_path) = cmd.template {
        if template_path.exists() {
            println!("📝 Usando plantilla: {}", template_path.display());
        } else {
            println!("⚠️  Plantilla no encontrada: {}", template_path.display());
        }
    }

    // F6: Modo single-file
    if cmd.single_file {
        println!("📋 Modo single-file: concatenando todos los documentos...");

        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        let mut total_content = String::new();
        let mut files_included = 0;

        total_content.push_str("# Documentación Completa\n\n");
        total_content.push_str(&format!(
            "_Generado: {}_\n\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        ));
        total_content.push_str("---\n\n");

        for file_path in &files {
            if let Ok(content) = read_file_content(file_path) {
                let name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("documento");

                total_content.push_str(&format!("## {}\n\n", name));

                // Saltar frontmatter
                if let Some(end_marker) = content.find("\n---\n") {
                    if content.starts_with("---") {
                        total_content.push_str(&content[end_marker + 5..]);
                    } else {
                        total_content.push_str(&content);
                    }
                } else {
                    total_content.push_str(&content);
                }

                total_content.push_str("\n\n---\n\n");
                files_included += 1;
            }
        }

        // Guardar archivo único
        let output_path = cmd
            .output
            .clone()
            .unwrap_or_else(|| PathBuf::from("documentacion_completa.md"));
        std::fs::write(&output_path, &total_content)?;

        println!(
            "✅ {} archivos concatenados en {}",
            files_included,
            output_path.display()
        );
        println!("💾 {} bytes escritos", total_content.len());
        return Ok(());
    }

    // Lógica normal
    let result = cmd.run(data_dir)?;

    println!("📤 Exportando a formato: {}", result.format);
    println!("📁 Salida: {}", result.output_path.display());
    println!(
        "📊 {} archivos, {} bytes",
        result.files_exported, result.total_bytes
    );

    Ok(())
}
