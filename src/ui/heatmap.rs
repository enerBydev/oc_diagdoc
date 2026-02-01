//! Visualización de heatmaps y barras de cobertura.
//!
//! Proporciona:
//! - Barras de progreso ASCII
//! - Heatmaps de cobertura por módulo
//! - Colorizado por niveles

use colored::Colorize;

// ═══════════════════════════════════════════════════════════════════════════
// CARACTERES DE BARRA
// ═══════════════════════════════════════════════════════════════════════════

const FULL_BLOCK: char = '█';
const LIGHT_BLOCK: char = '░';
#[allow(dead_code)]
const MED_BLOCK: char = '▒';
#[allow(dead_code)]
const EMPTY_BLOCK: char = ' ';

// ═══════════════════════════════════════════════════════════════════════════
// RENDERIZADO DE BARRAS
// ═══════════════════════════════════════════════════════════════════════════

/// Renderiza una barra de progreso ASCII.
pub fn render_bar(value: f64, max: f64, width: usize) -> String {
    let ratio = if max > 0.0 {
        (value / max).min(1.0)
    } else {
        0.0
    };
    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "{}{}",
        FULL_BLOCK.to_string().repeat(filled),
        LIGHT_BLOCK.to_string().repeat(empty)
    )
}

/// Renderiza barra con color según porcentaje.
pub fn render_colored_bar(percent: f64, width: usize) -> String {
    let bar = render_bar(percent, 100.0, width);

    if percent >= 90.0 {
        bar.green().to_string()
    } else if percent >= 70.0 {
        bar.bright_green().to_string()
    } else if percent >= 50.0 {
        bar.yellow().to_string()
    } else if percent >= 30.0 {
        bar.red().to_string()
    } else {
        bar.bright_red().to_string()
    }
}

/// Renderiza barra con etiqueta.
pub fn render_labeled_bar(label: &str, percent: f64, width: usize, label_width: usize) -> String {
    let bar = render_colored_bar(percent, width);
    let pct = format!("{:5.1}%", percent);
    let pct_colored = if percent >= 70.0 {
        pct.green()
    } else if percent >= 50.0 {
        pct.yellow()
    } else {
        pct.red()
    };

    format!(
        "{:width$} {} {}",
        label,
        bar,
        pct_colored,
        width = label_width
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// HEATMAP DE COBERTURA
// ═══════════════════════════════════════════════════════════════════════════

/// Datos de módulo para heatmap.
#[derive(Debug, Clone)]
pub struct ModuleHeat {
    pub name: String,
    pub coverage: f64,
    pub docs: usize,
    pub words: usize,
}

/// Renderiza un heatmap de módulos.
pub fn render_coverage_heatmap(modules: &[ModuleHeat], bar_width: usize) -> String {
    let max_name = modules.iter().map(|m| m.name.len()).max().unwrap_or(10);

    let mut lines = Vec::new();

    for module in modules {
        lines.push(render_labeled_bar(
            &module.name,
            module.coverage,
            bar_width,
            max_name,
        ));
    }

    lines.join("\n")
}

/// Renderiza una celda de heat individual.
pub fn heat_cell(value: f64) -> String {
    let c = if value >= 90.0 {
        "█".bright_green()
    } else if value >= 70.0 {
        "▓".green()
    } else if value >= 50.0 {
        "▒".yellow()
    } else if value >= 30.0 {
        "░".red()
    } else {
        " ".normal()
    };
    c.to_string()
}

/// Renderiza una fila de heat cells.
pub fn heat_row(values: &[f64]) -> String {
    values.iter().map(|v| heat_cell(*v)).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// MINI GRÁFICOS
// ═══════════════════════════════════════════════════════════════════════════

/// Sparkline simple.
pub fn sparkline(values: &[f64]) -> String {
    const CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let max = values.iter().cloned().fold(0.0_f64, f64::max);
    if max == 0.0 {
        return " ".repeat(values.len());
    }

    values
        .iter()
        .map(|v| {
            let idx = ((v / max) * 7.0).round() as usize;
            CHARS[idx.min(7)]
        })
        .collect()
}

/// Mini barra horizontal.
pub fn mini_bar(value: f64, max: f64) -> char {
    const CHARS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

    if max == 0.0 {
        return ' ';
    }
    let ratio = (value / max).min(1.0);
    let idx = (ratio * 8.0).round() as usize;
    CHARS[idx.min(8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_bar() {
        let bar = render_bar(50.0, 100.0, 10);
        assert_eq!(bar.chars().filter(|c| *c == FULL_BLOCK).count(), 5);
        assert_eq!(bar.chars().filter(|c| *c == LIGHT_BLOCK).count(), 5);
    }

    #[test]
    fn test_render_bar_full() {
        let bar = render_bar(100.0, 100.0, 10);
        assert_eq!(bar.chars().filter(|c| *c == FULL_BLOCK).count(), 10);
    }

    #[test]
    fn test_sparkline() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let spark = sparkline(&values);
        assert_eq!(spark.chars().count(), 5);
    }

    #[test]
    fn test_heat_cell() {
        assert!(heat_cell(95.0).contains('█'));
        assert!(heat_cell(75.0).contains('▓'));
    }
}
