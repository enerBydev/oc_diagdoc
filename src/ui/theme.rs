//! Sistema de temas y colores para la CLI de OnlyCar.
//!
//! Proporciona:
//! - Paleta de colores corporativa
//! - Funciones de formateo con colores
//! - Emojis semánticos
//! - Detección de soporte de colores

use colored::{Colorize, ColoredString};
use std::sync::atomic::{AtomicBool, Ordering};

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURACIÓN GLOBAL
// ═══════════════════════════════════════════════════════════════════════════

static COLORS_ENABLED: AtomicBool = AtomicBool::new(true);

/// Habilita o deshabilita colores globalmente.
pub fn set_colors_enabled(enabled: bool) {
    COLORS_ENABLED.store(enabled, Ordering::SeqCst);
}

/// ¿Están los colores habilitados?
pub fn colors_enabled() -> bool {
    COLORS_ENABLED.load(Ordering::SeqCst)
}

/// Detecta si el terminal soporta colores.
pub fn detect_color_support() -> bool {
    // Verificar variable de ambiente
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    if std::env::var("FORCE_COLOR").is_ok() {
        return true;
    }
    // Verificar si es TTY
    atty::is(atty::Stream::Stdout)
}

/// Inicializa el sistema de colores según capacidades del terminal.
pub fn init_colors() {
    let supported = detect_color_support();
    set_colors_enabled(supported);
    if !supported {
        colored::control::set_override(false);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PALETA DE COLORES ONLYCAR
// ═══════════════════════════════════════════════════════════════════════════

/// Colores semánticos.
pub mod colors {
    /// Color primario OnlyCar (azul corporativo).
    pub const PRIMARY: &str = "blue";
    /// Color secundario.
    pub const SECONDARY: &str = "cyan";
    /// Color de acento.
    pub const ACCENT: &str = "magenta";
    
    /// Éxito (verde).
    pub const SUCCESS: &str = "green";
    /// Advertencia (amarillo).
    pub const WARNING: &str = "yellow";
    /// Error (rojo).
    pub const ERROR: &str = "red";
    /// Información (cian).
    pub const INFO: &str = "cyan";
    
    /// Texto atenuado.
    pub const DIM: &str = "bright black";
    /// Texto destacado.
    pub const HIGHLIGHT: &str = "white";
}

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES DE FORMATEO
// ═══════════════════════════════════════════════════════════════════════════

/// Aplica color a un texto.
pub fn colorize(text: &str, color: &str) -> ColoredString {
    match color {
        "blue" => text.blue(),
        "cyan" => text.cyan(),
        "magenta" => text.magenta(),
        "green" => text.green(),
        "yellow" => text.yellow(),
        "red" => text.red(),
        "white" => text.white(),
        "bright black" => text.bright_black(),
        _ => text.normal(),
    }
}

/// Texto en negrita.
pub fn bold(text: &str) -> ColoredString {
    text.bold()
}

/// Texto atenuado (dim).
pub fn dim(text: &str) -> ColoredString {
    text.dimmed()
}

/// Texto en cursiva.
pub fn italic(text: &str) -> ColoredString {
    text.italic()
}

/// Texto subrayado.
pub fn underline(text: &str) -> ColoredString {
    text.underline()
}

// ═══════════════════════════════════════════════════════════════════════════
// FORMATEO SEMÁNTICO
// ═══════════════════════════════════════════════════════════════════════════

/// Texto de éxito (verde).
pub fn success(text: &str) -> ColoredString {
    text.green().bold()
}

/// Texto de advertencia (amarillo).
pub fn warning(text: &str) -> ColoredString {
    text.yellow()
}

/// Texto de error (rojo).
pub fn error(text: &str) -> ColoredString {
    text.red().bold()
}

/// Texto informativo (cian).
pub fn info(text: &str) -> ColoredString {
    text.cyan()
}

/// Texto primario (azul).
pub fn primary(text: &str) -> ColoredString {
    text.blue().bold()
}

// ═══════════════════════════════════════════════════════════════════════════
// EMOJIS SEMÁNTICOS
// ═══════════════════════════════════════════════════════════════════════════

/// Lookup de emojis por nombre semántico.
pub fn emoji(name: &str) -> &'static str {
    match name {
        // Estados
        "success" | "ok" | "check" => "✅",
        "error" | "fail" | "x" => "❌",
        "warning" | "warn" => "⚠️",
        "info" | "i" => "ℹ️",
        "question" | "?" => "❓",
        
        // Progreso
        "loading" | "spinner" => "⏳",
        "done" | "complete" => "✨",
        "wip" | "working" => "🔄",
        "pending" => "⏸️",
        
        // Documentos
        "doc" | "document" | "file" => "📄",
        "folder" | "dir" => "📁",
        "link" => "🔗",
        "broken" => "💔",
        
        // Análisis
        "search" | "find" => "🔍",
        "stats" | "chart" => "📊",
        "tree" => "🌳",
        "graph" => "📈",
        
        // Acciones
        "add" | "plus" => "➕",
        "remove" | "minus" => "➖",
        "edit" | "pencil" => "✏️",
        "save" => "💾",
        "delete" | "trash" => "🗑️",
        
        // OnlyCar específicos
        "car" => "🚗",
        "nuclear" => "☢️",
        "rust" => "🦀",
        "atom" => "⚛️",
        
        // Misc
        "rocket" => "🚀",
        "fire" => "🔥",
        "star" => "⭐",
        "party" => "🎉",
        "bug" => "🐛",
        "fix" => "🔧",
        
        _ => "•",
    }
}

/// Icono con texto.
pub fn icon(name: &str, text: &str) -> String {
    format!("{} {}", emoji(name), text)
}

// ═══════════════════════════════════════════════════════════════════════════
// FORMATEO DE ESTADOS
// ═══════════════════════════════════════════════════════════════════════════

/// Formatea un estado de documento.
pub fn format_status(status: &str) -> ColoredString {
    match status.to_lowercase().as_str() {
        "active" => "ACTIVE".green(),
        "draft" => "DRAFT".yellow(),
        "deprecated" => "DEPRECATED".red().dimmed(),
        "archived" => "ARCHIVED".bright_black(),
        "reviewed" => "REVIEWED".blue(),
        _ => status.normal(),
    }
}

/// Formatea un porcentaje con color según umbral.
pub fn format_percent(value: f64) -> ColoredString {
    let text = format!("{:.1}%", value);
    if value >= 90.0 {
        text.green().bold()
    } else if value >= 70.0 {
        text.green()
    } else if value >= 50.0 {
        text.yellow()
    } else if value >= 30.0 {
        text.red()
    } else {
        text.red().bold()
    }
}

/// Formatea un conteo con color según umbral.
pub fn format_count(value: usize, warning_threshold: usize, error_threshold: usize) -> ColoredString {
    let text = value.to_string();
    if value >= error_threshold {
        text.red().bold()
    } else if value >= warning_threshold {
        text.yellow()
    } else {
        text.green()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_lookup() {
        assert_eq!(emoji("success"), "✅");
        assert_eq!(emoji("error"), "❌");
        assert_eq!(emoji("rust"), "🦀");
        assert_eq!(emoji("unknown"), "•");
    }

    #[test]
    fn test_format_status() {
        let status = format_status("active");
        assert!(status.to_string().contains("ACTIVE"));
    }

    #[test]
    fn test_format_percent() {
        let p = format_percent(95.5);
        assert!(p.to_string().contains("95.5%"));
    }

    #[test]
    fn test_icon() {
        let result = icon("success", "Completado");
        assert!(result.contains("✅"));
        assert!(result.contains("Completado"));
    }
}
