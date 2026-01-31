//! Sistema de tablas para output CLI.
//!
//! Proporciona:
//! - Wrapper sobre comfy-table
//! - Estilos predefinidos OnlyCar
//! - Funciones de creación y formateo

use comfy_table::{Table, ContentArrangement, presets, modifiers};
use crate::ui::theme::{format_status, format_percent, format_count};

// ═══════════════════════════════════════════════════════════════════════════
// ESTILOS PREDEFINIDOS
// ═══════════════════════════════════════════════════════════════════════════

/// Estilos de tabla predefinidos.
#[derive(Debug, Clone, Copy, Default)]
pub enum TableStyle {
    /// Estilo completo con bordes.
    Full,
    /// Estilo compacto sin bordes laterales.
    #[default]
    Compact,
    /// Estilo mínimo solo con separadores.
    Minimal,
    /// Estilo limpio sin bordes.
    Clean,
}

// ═══════════════════════════════════════════════════════════════════════════
// BUILDER DE TABLAS
// ═══════════════════════════════════════════════════════════════════════════

/// Crea una nueva tabla con headers.
pub fn create_table<S: AsRef<str>>(headers: &[S]) -> Table {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.load_preset(presets::UTF8_FULL);
    table.apply_modifier(modifiers::UTF8_ROUND_CORNERS);
    
    let headers: Vec<&str> = headers.iter().map(|s| s.as_ref()).collect();
    table.set_header(headers);
    
    table
}

/// Crea tabla con estilo específico.
pub fn create_styled_table<S: AsRef<str>>(headers: &[S], style: TableStyle) -> Table {
    let mut table = create_table(headers);
    
    match style {
        TableStyle::Full => {
            table.load_preset(presets::UTF8_FULL);
        }
        TableStyle::Compact => {
            table.load_preset(presets::UTF8_FULL_CONDENSED);
        }
        TableStyle::Minimal => {
            table.load_preset(presets::UTF8_HORIZONTAL_ONLY);
        }
        TableStyle::Clean => {
            table.load_preset(presets::NOTHING);
        }
    }
    
    table
}

/// Agrega una fila a la tabla.
pub fn add_row<S: ToString>(table: &mut Table, values: &[S]) {
    let row: Vec<String> = values.iter().map(|v| v.to_string()).collect();
    table.add_row(row);
}

/// Imprime la tabla.
pub fn print_table(table: &Table) {
    println!("{}", table);
}

// ═══════════════════════════════════════════════════════════════════════════
// TABLAS ESPECIALIZADAS
// ═══════════════════════════════════════════════════════════════════════════

/// Crea tabla de resumen de módulos.
pub fn create_module_table() -> Table {
    create_styled_table(
        &["Módulo", "Docs", "Palabras", "Cobertura", "Estado"],
        TableStyle::Compact,
    )
}

/// Agrega fila de módulo.
pub fn add_module_row(
    table: &mut Table,
    module: &str,
    docs: usize,
    words: usize,
    coverage: f64,
    status: &str,
) {
    table.add_row(vec![
        module.to_string(),
        docs.to_string(),
        format_word_count(words),
        format_percent(coverage).to_string(),
        format_status(status).to_string(),
    ]);
}

/// Crea tabla de diagnóstico.
pub fn create_diagnostic_table() -> Table {
    create_styled_table(
        &["Tipo", "Archivo", "Mensaje", "Severidad"],
        TableStyle::Compact,
    )
}

/// Crea tabla de estadísticas.
pub fn create_stats_table() -> Table {
    create_styled_table(
        &["Métrica", "Valor", "Descripción"],
        TableStyle::Minimal,
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// FORMATEO
// ═══════════════════════════════════════════════════════════════════════════

/// Formatea conteo de palabras con separador de miles.
pub fn format_word_count(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

/// Formatea tamaño de archivo.
pub fn format_file_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let table = create_table(&["Col1", "Col2"]);
        let output = table.to_string();
        assert!(output.contains("Col1"));
        assert!(output.contains("Col2"));
    }

    #[test]
    fn test_add_row() {
        let mut table = create_table(&["A", "B"]);
        add_row(&mut table, &["val1", "val2"]);
        let output = table.to_string();
        assert!(output.contains("val1"));
    }

    #[test]
    fn test_format_word_count() {
        assert_eq!(format_word_count(500), "500");
        assert_eq!(format_word_count(1500), "1.5K");
        assert_eq!(format_word_count(1_500_000), "1.5M");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 B");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1_572_864), "1.5 MB");
    }
}
