//! Sistema de barras de progreso para CLI.
//!
//! Proporciona:
//! - Wrapper sobre indicatif
//! - Templates OnlyCar predefinidos
//! - Spinners y multi-progress

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════
// TEMPLATES ONLYCAR
// ═══════════════════════════════════════════════════════════════════════════

/// Template estándar de barra de progreso.
pub const TEMPLATE_STANDARD: &str =
    "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}";

/// Template compacto.
pub const TEMPLATE_COMPACT: &str = "{spinner:.cyan} {bar:30.cyan/blue} {pos}/{len} {msg}";

/// Template para bytes/archivos.
pub const TEMPLATE_BYTES: &str =
    "{spinner:.green} [{bar:40.green/white}] {bytes}/{total_bytes} ({bytes_per_sec}) {msg}";

/// Template de spinner.
pub const TEMPLATE_SPINNER: &str = "{spinner:.cyan} {msg}";

// ═══════════════════════════════════════════════════════════════════════════
// CREACIÓN DE BARRAS DE PROGRESO
// ═══════════════════════════════════════════════════════════════════════════

/// Crea una barra de progreso estándar.
pub fn create_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(TEMPLATE_STANDARD)
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Crea una barra de progreso con template custom.
pub fn create_progress_bar_with_template(total: u64, template: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Crea una barra compacta.
pub fn create_compact_bar(total: u64) -> ProgressBar {
    create_progress_bar_with_template(total, TEMPLATE_COMPACT)
}

/// Crea una barra para bytes.
pub fn create_bytes_bar(total: u64) -> ProgressBar {
    create_progress_bar_with_template(total, TEMPLATE_BYTES)
}

// ═══════════════════════════════════════════════════════════════════════════
// SPINNERS
// ═══════════════════════════════════════════════════════════════════════════

/// Crea un spinner con mensaje.
pub fn create_spinner(msg: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template(TEMPLATE_SPINNER)
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    spinner.set_message(msg.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

/// Spinner con estilo de puntos.
pub fn create_dots_spinner(msg: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷ "),
    );
    spinner.set_message(msg.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

// ═══════════════════════════════════════════════════════════════════════════
// MULTI-PROGRESS
// ═══════════════════════════════════════════════════════════════════════════

/// Crea un multi-progress para múltiples barras.
pub fn create_multi_progress() -> MultiProgress {
    MultiProgress::new()
}

/// Crea multi-progress oculto (para tests).
pub fn create_hidden_multi_progress() -> MultiProgress {
    let mp = MultiProgress::new();
    mp.set_draw_target(ProgressDrawTarget::hidden());
    mp
}

/// Agrega una barra al multi-progress.
pub fn add_bar_to_multi(mp: &MultiProgress, total: u64) -> ProgressBar {
    let pb = create_progress_bar(total);
    mp.add(pb)
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Barra de progreso silenciosa (para tests o modo quiet).
pub fn create_hidden_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_draw_target(ProgressDrawTarget::hidden());
    pb
}

/// Finaliza la barra con mensaje de éxito.
pub fn finish_with_success(pb: &ProgressBar, msg: &str) {
    pb.finish_with_message(format!("✅ {}", msg));
}

/// Finaliza la barra con mensaje de error.
pub fn finish_with_error(pb: &ProgressBar, msg: &str) {
    pb.finish_with_message(format!("❌ {}", msg));
}

/// Incrementa y actualiza mensaje.
pub fn inc_with_message(pb: &ProgressBar, delta: u64, msg: &str) {
    pb.inc(delta);
    pb.set_message(msg.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_progress_bar() {
        let pb = create_hidden_bar(100);
        assert_eq!(pb.length(), Some(100));
    }

    #[test]
    fn test_create_spinner() {
        let spinner = create_spinner("Loading...");
        assert!(spinner.message().contains("Loading"));
    }

    #[test]
    fn test_multi_progress() {
        let mp = create_hidden_multi_progress();
        let pb = add_bar_to_multi(&mp, 50);
        assert_eq!(pb.length(), Some(50));
    }

    #[test]
    fn test_inc_with_message() {
        let pb = create_hidden_bar(100);
        inc_with_message(&pb, 10, "Processing...");
        assert_eq!(pb.position(), 10);
    }
}
