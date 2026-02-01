//! Sistema de logging con soporte para colores y niveles.
//!
//! Proporciona:
//! - Macros para diferentes niveles de log
//! - Logger configurable
//! - Output a stderr para compatibilidad con pipes

use colored::Colorize;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU8, Ordering};

use crate::ui::theme::{emoji, error, info, success, warning};

// ═══════════════════════════════════════════════════════════════════════════
// NIVEL DE LOG
// ═══════════════════════════════════════════════════════════════════════════

/// Niveles de log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Quiet = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl From<u8> for LogLevel {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Quiet,
            1 => Self::Error,
            2 => Self::Warning,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

// Nivel global
static LOG_LEVEL: AtomicU8 = AtomicU8::new(3); // Info por defecto

/// Establece el nivel de log global.
pub fn set_log_level(level: LogLevel) {
    LOG_LEVEL.store(level as u8, Ordering::SeqCst);
}

/// Obtiene el nivel de log actual.
pub fn get_log_level() -> LogLevel {
    LogLevel::from(LOG_LEVEL.load(Ordering::SeqCst))
}

/// ¿Debería loggear a este nivel?
pub fn should_log(level: LogLevel) -> bool {
    level <= get_log_level()
}

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES DE LOG
// ═══════════════════════════════════════════════════════════════════════════

/// Log de información.
pub fn log_info(msg: &str) {
    if should_log(LogLevel::Info) {
        eprintln!("{} {}", emoji("info").blue(), msg);
    }
}

/// Log de éxito.
pub fn log_success(msg: &str) {
    if should_log(LogLevel::Info) {
        eprintln!("{} {}", emoji("success"), success(msg));
    }
}

/// Log de advertencia.
pub fn log_warning(msg: &str) {
    if should_log(LogLevel::Warning) {
        eprintln!("{} {}", emoji("warning"), warning(msg));
    }
}

/// Log de error.
pub fn log_error(msg: &str) {
    if should_log(LogLevel::Error) {
        eprintln!("{} {}", emoji("error"), error(msg));
    }
}

/// Log de debug.
pub fn log_debug(msg: &str) {
    if should_log(LogLevel::Debug) {
        eprintln!("{} {}", "DEBUG:".bright_black(), msg.bright_black());
    }
}

/// Log de acción (describe lo que se está haciendo).
pub fn log_action(msg: &str) {
    if should_log(LogLevel::Info) {
        eprintln!("{} {}", emoji("wip"), info(msg));
    }
}

/// Log de paso en un proceso.
pub fn log_step(current: usize, total: usize, msg: &str) {
    if should_log(LogLevel::Info) {
        let prefix = format!("[{}/{}]", current, total);
        eprintln!("{} {}", prefix.bright_black(), msg);
    }
}

/// Log con sangría (para sub-items).
pub fn log_indent(msg: &str, level: usize) {
    if should_log(LogLevel::Info) {
        let indent = "  ".repeat(level);
        eprintln!("{}• {}", indent, msg);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LOGGER STRUCT
// ═══════════════════════════════════════════════════════════════════════════

/// Logger configurable.
#[derive(Debug, Clone)]
pub struct Logger {
    prefix: Option<String>,
    level: LogLevel,
    use_stderr: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            prefix: None,
            level: LogLevel::Info,
            use_stderr: true,
        }
    }
}

impl Logger {
    /// Crea un nuevo logger.
    pub fn new() -> Self {
        Self::default()
    }

    /// Con prefijo.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Con nivel.
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Usa stdout en lugar de stderr.
    pub fn use_stdout(mut self) -> Self {
        self.use_stderr = false;
        self
    }

    fn format_msg(&self, msg: &str) -> String {
        match &self.prefix {
            Some(p) => format!("[{}] {}", p, msg),
            None => msg.to_string(),
        }
    }

    fn write(&self, msg: &str) {
        if self.use_stderr {
            let _ = writeln!(io::stderr(), "{}", msg);
        } else {
            println!("{}", msg);
        }
    }

    /// Log info.
    pub fn info(&self, msg: &str) {
        if self.level >= LogLevel::Info {
            self.write(&format!(
                "{} {}",
                emoji("info").blue(),
                self.format_msg(msg)
            ));
        }
    }

    /// Log success.
    pub fn success(&self, msg: &str) {
        if self.level >= LogLevel::Info {
            self.write(&format!(
                "{} {}",
                emoji("success"),
                success(&self.format_msg(msg))
            ));
        }
    }

    /// Log warning.
    pub fn warning(&self, msg: &str) {
        if self.level >= LogLevel::Warning {
            self.write(&format!(
                "{} {}",
                emoji("warning"),
                warning(&self.format_msg(msg))
            ));
        }
    }

    /// Log error.
    pub fn error(&self, msg: &str) {
        if self.level >= LogLevel::Error {
            self.write(&format!(
                "{} {}",
                emoji("error"),
                error(&self.format_msg(msg))
            ));
        }
    }

    /// Log debug.
    pub fn debug(&self, msg: &str) {
        if self.level >= LogLevel::Debug {
            self.write(&format!(
                "{} {}",
                "DEBUG:".bright_black(),
                self.format_msg(msg).bright_black()
            ));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MACROS
// ═══════════════════════════════════════════════════════════════════════════

/// Macro para log info.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::ui::logger::log_info(&format!($($arg)*))
    };
}

/// Macro para log success.
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        $crate::ui::logger::log_success(&format!($($arg)*))
    };
}

/// Macro para log warning.
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::ui::logger::log_warning(&format!($($arg)*))
    };
}

/// Macro para log error.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::ui::logger::log_error(&format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
    }

    #[test]
    fn test_should_log() {
        set_log_level(LogLevel::Warning);
        assert!(should_log(LogLevel::Error));
        assert!(should_log(LogLevel::Warning));
        assert!(!should_log(LogLevel::Info));

        // Restaurar
        set_log_level(LogLevel::Info);
    }

    #[test]
    fn test_logger_with_prefix() {
        let logger = Logger::new().with_prefix("TEST");
        let msg = logger.format_msg("hello");
        assert!(msg.contains("[TEST]"));
        assert!(msg.contains("hello"));
    }

    #[test]
    fn test_log_level_from_u8() {
        assert_eq!(LogLevel::from(0), LogLevel::Quiet);
        assert_eq!(LogLevel::from(3), LogLevel::Info);
        assert_eq!(LogLevel::from(99), LogLevel::Trace);
    }
}
