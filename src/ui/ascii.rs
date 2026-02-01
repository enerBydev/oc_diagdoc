//! ASCII Art y banners para CLI.
//!
//! Proporciona:
//! - Logo OnlyCar ASCII
//! - Banners de versión
//! - Separadores y box drawing

use colored::Colorize;

// ═══════════════════════════════════════════════════════════════════════════
// LOGO ASCII
// ═══════════════════════════════════════════════════════════════════════════

/// Logo ASCII de oc_diagdoc.
pub const LOGO: &str = r#"
   ____   ____   ____  _____    _    ____ ____   ___   ____ 
  / __ \ / ___| |  _ \|_ _  |  / \  / ___|  _ \ / _ \ / ___|
 | |  | | |     | | | || |  | / _ \| |  _| | | | | | | |    
 | |__| | |___  | |_| || |  |/ ___ \ |_| | |_| | |_| | |___ 
  \____/ \____| |____/|_____|/_/   \_\____|____/ \___/ \____|
"#;

/// Logo compacto.
pub const LOGO_COMPACT: &str = "🦀⚛️☢️ oc_diagdoc";

// ═══════════════════════════════════════════════════════════════════════════
// BANNERS
// ═══════════════════════════════════════════════════════════════════════════

/// Imprime el banner con versión.
pub fn print_banner(version: &str) {
    println!("{}", LOGO.cyan());
    println!(
        "{}",
        format!("    v{} - Nuclear Documentation Diagnostics", version).bright_black()
    );
    println!();
}

/// Imprime banner compacto.
pub fn print_compact_banner(version: &str) {
    println!(
        "{} {} {}",
        LOGO_COMPACT,
        format!("v{}", version).cyan(),
        "Nuclear Documentation Diagnostics".bright_black()
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// SEPARADORES
// ═══════════════════════════════════════════════════════════════════════════

/// Separador simple.
pub fn separator(width: usize) -> String {
    "─".repeat(width)
}

/// Separador doble.
pub fn double_separator(width: usize) -> String {
    "═".repeat(width)
}

/// Separador con título.
pub fn titled_separator(title: &str, width: usize) -> String {
    let title_len = title.chars().count();
    if title_len + 4 >= width {
        return format!("─ {} ─", title);
    }

    let remaining = width - title_len - 4;
    let left = remaining / 2;
    let right = remaining - left;

    format!("{}─ {} ─{}", "─".repeat(left), title, "─".repeat(right))
}

/// Imprime separador.
pub fn print_separator() {
    println!("{}", separator(60).bright_black());
}

/// Imprime separador doble.
pub fn print_double_separator() {
    println!("{}", double_separator(60).bright_black());
}

// ═══════════════════════════════════════════════════════════════════════════
// BOX DRAWING
// ═══════════════════════════════════════════════════════════════════════════

/// Esquinas y líneas para boxes.
pub mod box_chars {
    pub const TOP_LEFT: char = '┌';
    pub const TOP_RIGHT: char = '┐';
    pub const BOTTOM_LEFT: char = '└';
    pub const BOTTOM_RIGHT: char = '┘';
    pub const HORIZONTAL: char = '─';
    pub const VERTICAL: char = '│';
    pub const T_DOWN: char = '┬';
    pub const T_UP: char = '┴';
    pub const T_LEFT: char = '┤';
    pub const T_RIGHT: char = '├';
    pub const CROSS: char = '┼';

    // Doble línea
    pub const D_TOP_LEFT: char = '╔';
    pub const D_TOP_RIGHT: char = '╗';
    pub const D_BOTTOM_LEFT: char = '╚';
    pub const D_BOTTOM_RIGHT: char = '╝';
    pub const D_HORIZONTAL: char = '═';
    pub const D_VERTICAL: char = '║';
}

/// Crea una caja de texto simple.
pub fn text_box(text: &str, width: usize) -> String {
    use box_chars::*;

    let inner_width = width.saturating_sub(2);
    let padded = format!("{:<width$}", text, width = inner_width);

    let top = format!(
        "{}{}{}",
        TOP_LEFT,
        HORIZONTAL.to_string().repeat(inner_width),
        TOP_RIGHT
    );
    let middle = format!("{}{}{}", VERTICAL, padded, VERTICAL);
    let bottom = format!(
        "{}{}{}",
        BOTTOM_LEFT,
        HORIZONTAL.to_string().repeat(inner_width),
        BOTTOM_RIGHT
    );

    format!("{}\n{}\n{}", top, middle, bottom)
}

/// Crea una caja con título.
pub fn titled_box(title: &str, content: &str, width: usize) -> String {
    use box_chars::*;

    let inner_width = width.saturating_sub(2);
    let title_line = format!(" {} ", title);
    let title_padding = inner_width.saturating_sub(title_line.len());

    let top = format!(
        "{}{}{}{}",
        TOP_LEFT,
        title_line,
        HORIZONTAL.to_string().repeat(title_padding),
        TOP_RIGHT
    );

    let mut lines = vec![top];

    for line in content.lines() {
        let padded = format!("{:<width$}", line, width = inner_width);
        lines.push(format!("{}{}{}", VERTICAL, padded, VERTICAL));
    }

    lines.push(format!(
        "{}{}{}",
        BOTTOM_LEFT,
        HORIZONTAL.to_string().repeat(inner_width),
        BOTTOM_RIGHT
    ));

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator() {
        let sep = separator(10);
        assert_eq!(sep.len(), 30); // 10 chars × 3 bytes per ─
    }

    #[test]
    fn test_titled_separator() {
        let sep = titled_separator("Test", 20);
        assert!(sep.contains("Test"));
        assert!(sep.contains("─"));
    }

    #[test]
    fn test_text_box() {
        let box_str = text_box("Hello", 10);
        assert!(box_str.contains("Hello"));
        assert!(box_str.contains("┌"));
        assert!(box_str.contains("└"));
    }

    #[test]
    fn test_titled_box() {
        let box_str = titled_box("Title", "Content here", 20);
        assert!(box_str.contains("Title"));
        assert!(box_str.contains("Content"));
    }
}
