//! Trait Exportable - Para exportación de documentos a diferentes formatos.

use std::path::PathBuf;

/// Formato de exportación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown,
    Html,
    Pdf,
    Json,
    Csv,
    Docx,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Markdown => "md",
            ExportFormat::Html => "html",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Json => "json",
            ExportFormat::Csv => "csv",
            ExportFormat::Docx => "docx",
        }
    }
    
    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Markdown => "text/markdown",
            ExportFormat::Html => "text/html",
            ExportFormat::Pdf => "application/pdf",
            ExportFormat::Json => "application/json",
            ExportFormat::Csv => "text/csv",
            ExportFormat::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        }
    }
}

/// Opciones de exportación.
#[derive(Debug, Clone, Default)]
pub struct ExportOptions {
    pub include_metadata: bool,
    pub include_toc: bool,
    pub template: Option<String>,
}

/// Trait para elementos que pueden ser exportados.
pub trait Exportable {
    /// Exporta a formato especificado.
    fn export(&self, format: ExportFormat) -> Result<Vec<u8>, ExportError>;
    
    /// Exporta a archivo.
    fn export_to_file(&self, path: PathBuf, format: ExportFormat) -> Result<(), ExportError> {
        let data = self.export(format)?;
        std::fs::write(&path, &data).map_err(|e| ExportError::IoError(e.to_string()))
    }
    
    /// Formatos soportados.
    fn supported_formats(&self) -> Vec<ExportFormat> {
        vec![ExportFormat::Markdown, ExportFormat::Json]
    }
    
    /// ¿Soporta el formato?
    fn supports(&self, format: &ExportFormat) -> bool {
        self.supported_formats().contains(format)
    }
}

/// Error de exportación.
#[derive(Debug)]
pub enum ExportError {
    UnsupportedFormat(ExportFormat),
    IoError(String),
    ConversionError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::UnsupportedFormat(fmt) => write!(f, "Unsupported format: {:?}", fmt),
            ExportError::IoError(msg) => write!(f, "IO error: {}", msg),
            ExportError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_extension() {
        assert_eq!(ExportFormat::Markdown.extension(), "md");
        assert_eq!(ExportFormat::Pdf.extension(), "pdf");
    }
    
    #[test]
    fn test_mime_type() {
        assert_eq!(ExportFormat::Json.mime_type(), "application/json");
    }
}
