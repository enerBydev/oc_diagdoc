//! Sistema de temas y colores para la CLI de OnlyCar.
//!
//! Proporciona:
//! - Paleta de colores corporativa
//! - Funciones de formateo con colores
//! - Emojis semánticos
//! - Detección de soporte de colores

use colored::{ColoredString, Colorize};
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
// F8: BANNER Y MEJORAS UX
// ═══════════════════════════════════════════════════════════════════════════

/// Banner ASCII de OnlyCar.
pub fn print_banner() {
    let version = env!("CARGO_PKG_VERSION");
    let banner = format!(r#"
╔═══════════════════════════════════════════════════════════════════════════╗
║   ___              _         _____              _   _  _     ____         ║
║  / _ \  _ __  | | _   _ / ____|__ _ _ __   | \ | || |   |  _ \         ║
║ | | | || '_ \ | || | | || |    / _` || '__|  |  \| || |   | | | |        ║
║ | |_| || | | || || |_| || |___| (_| || |     | |\  || |___| |_| |        ║
║  \___/ |_| |_||_| \__, | \_____\__,_||_|     |_| \_||_____|____/         ║
║                    |___/                                                   ║
║                                                                           ║
║   🚗 oc_diagdoc v{}  |  Motor Algorítmico Nuclear  |  🦀 Rust Puro    ║
╚═══════════════════════════════════════════════════════════════════════════╝
"#, version);
    println!("{}", banner.cyan());
}

/// Banner compacto.
pub fn print_banner_compact() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        "══════════════════════════════════════════════".cyan()
    );
    println!(
        "  {} {} {}",
        "🚗".to_string(),
        format!("oc_diagdoc v{}", version).blue().bold(),
        "| Motor Algorítmico Nuclear".dimmed()
    );
    println!(
        "{}",
        "══════════════════════════════════════════════".cyan()
    );
}

/// Imprime un separador.
pub fn print_separator() {
    println!(
        "{}",
        "═══════════════════════════════════════════════════════".dimmed()
    );
}

/// Imprime un separador ligero.
pub fn print_separator_light() {
    println!(
        "{}",
        "───────────────────────────────────────────────────────".bright_black()
    );
}

/// Imprime header de sección.
pub fn print_section_header(title: &str) {
    println!();
    println!("{} {}", "▶".cyan(), title.bold());
    println!("{}", "─".repeat(50).bright_black());
}

/// Imprime un resumen de operación.
pub fn print_summary(label: &str, value: &str, status_ok: bool) {
    let status_icon = if status_ok { "✅" } else { "⚠️" };
    let value_colored = if status_ok {
        value.green().to_string()
    } else {
        value.yellow().to_string()
    };
    println!("  {} {}: {}", status_icon, label.dimmed(), value_colored);
}

/// Imprime estadística formateada.
pub fn print_stat(label: &str, value: usize, suffix: &str) {
    println!(
        "  {} {}: {} {}",
        "•".cyan(),
        label,
        value.to_string().blue().bold(),
        suffix.dimmed()
    );
}

/// Imprime error formateado.
pub fn print_error_box(title: &str, message: &str) {
    println!();
    println!(
        "{}",
        "┌─ ❌ ERROR ─────────────────────────────────────────┐".red()
    );
    println!("{} {}", "│".red(), title.red().bold());
    println!("{} {}", "│".red(), message);
    println!(
        "{}",
        "└────────────────────────────────────────────────────┘".red()
    );
}

/// Imprime warning formateado.
pub fn print_warning_box(title: &str, message: &str) {
    println!();
    println!(
        "{}",
        "┌─ ⚠️ WARNING ──────────────────────────────────────┐".yellow()
    );
    println!("{} {}", "│".yellow(), title.yellow().bold());
    println!("{} {}", "│".yellow(), message);
    println!(
        "{}",
        "└────────────────────────────────────────────────────┘".yellow()
    );
}

/// Imprime success box.
pub fn print_success_box(title: &str, message: &str) {
    println!();
    println!(
        "{}",
        "┌─ ✅ SUCCESS ──────────────────────────────────────┐".green()
    );
    println!("{} {}", "│".green(), title.green().bold());
    println!("{} {}", "│".green(), message);
    println!(
        "{}",
        "└────────────────────────────────────────────────────┘".green()
    );
}

/// Imprime tabla de stats resumida.
pub fn print_stats_table(items: &[(&str, usize)]) {
    println!();
    println!(
        "{}",
        "┌────────────────────────────────┬──────────┐".dimmed()
    );
    println!(
        "{} {:^30} {} {:^8} {}",
        "│".dimmed(),
        "Métrica".bold(),
        "│".dimmed(),
        "Valor".bold(),
        "│".dimmed()
    );
    println!(
        "{}",
        "├────────────────────────────────┼──────────┤".dimmed()
    );
    for (label, value) in items {
        println!(
            "{} {:30} {} {:>8} {}",
            "│".dimmed(),
            label,
            "│".dimmed(),
            value.to_string().cyan(),
            "│".dimmed()
        );
    }
    println!(
        "{}",
        "└────────────────────────────────┴──────────┘".dimmed()
    );
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
pub fn format_count(
    value: usize,
    warning_threshold: usize,
    error_threshold: usize,
) -> ColoredString {
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
