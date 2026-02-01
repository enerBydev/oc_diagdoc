//! Trait para renderizado multi-formato.

use serde::Serialize;

// ═══════════════════════════════════════════════════════════════════════════
// OUTPUT FORMAT
// ═══════════════════════════════════════════════════════════════════════════

/// Formatos de output soportados.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Tabla ASCII.
    #[default]
    Table,
    /// JSON.
    Json,
    /// Markdown.
    Markdown,
    /// CSV.
    Csv,
    /// Texto plano.
    Plain,
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT RENDERABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos renderizables en múltiples formatos.
pub trait Renderable: Serialize {
    /// Renderiza en formato tabla (para terminal).
    fn render_table(&self) -> String;

    /// Renderiza como JSON.
    fn render_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Renderiza como Markdown.
    fn render_markdown(&self) -> String;

    /// Renderiza como CSV.
    fn render_csv(&self) -> String {
        String::new() // Default vacío
    }

    /// Renderiza como texto plano.
    fn render_plain(&self) -> String {
        self.render_table()
    }

    /// Renderiza en el formato especificado.
    fn render(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Table => self.render_table(),
            OutputFormat::Json => self.render_json(),
            OutputFormat::Markdown => self.render_markdown(),
            OutputFormat::Csv => self.render_csv(),
            OutputFormat::Plain => self.render_plain(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Escapa caracteres especiales para Markdown.
pub fn escape_markdown(text: &str) -> String {
    text.replace('|', "\\|")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
}

/// Formatea como celda de tabla Markdown.
pub fn md_cell(text: &str) -> String {
    escape_markdown(text.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_default() {
        let format = OutputFormat::default();
        assert_eq!(format, OutputFormat::Table);
    }

    #[test]
    fn test_escape_markdown() {
        let text = "hello | world * _test_";
        let escaped = escape_markdown(text);
        assert!(escaped.contains("\\|"));
        assert!(escaped.contains("\\*"));
    }

    #[test]
    fn test_md_cell() {
        let cell = md_cell("  value  ");
        assert_eq!(cell, "value");
    }
}
