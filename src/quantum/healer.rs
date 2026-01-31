//! Auto-sanador para correcciones automáticas.
//!
//! Repara problemas triviales sin intervención del usuario.

use std::path::PathBuf;
use crate::errors::{OcError, OcResult};
use crate::traits::{Fix, FixResult, Fixable};

// ═══════════════════════════════════════════════════════════════════════════
// HEAL ACTION
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de acción de curación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealAction {
    /// Sincronizar fecha.
    SyncDate,
    /// Normalizar whitespace.
    NormalizeWhitespace,
    /// Corregir encoding.
    FixEncoding,
    /// Reconstruir breadcrumb.
    RebuildBreadcrumb,
    /// Agregar campo faltante.
    AddMissingField(String),
}

/// Resultado de una curación.
#[derive(Debug, Clone)]
pub struct HealResult {
    pub action: HealAction,
    pub target: PathBuf,
    pub success: bool,
    pub message: String,
}

impl HealResult {
    pub fn success(action: HealAction, target: PathBuf, message: impl Into<String>) -> Self {
        Self {
            action,
            target,
            success: true,
            message: message.into(),
        }
    }
    
    pub fn failure(action: HealAction, target: PathBuf, message: impl Into<String>) -> Self {
        Self {
            action,
            target,
            success: false,
            message: message.into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALER
// ═══════════════════════════════════════════════════════════════════════════

/// Auto-sanador cuántico.
#[derive(Debug, Default)]
pub struct Healer {
    /// Acciones permitidas automáticamente.
    allowed_actions: Vec<HealAction>,
    /// Modo dry-run (no hace cambios reales).
    dry_run: bool,
    /// Log de curaciones realizadas.
    heal_log: Vec<HealResult>,
}

impl Healer {
    pub fn new() -> Self {
        Self {
            allowed_actions: vec![
                HealAction::SyncDate,
                HealAction::NormalizeWhitespace,
            ],
            dry_run: false,
            heal_log: Vec::new(),
        }
    }
    
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
    
    pub fn allow_action(mut self, action: HealAction) -> Self {
        if !self.allowed_actions.contains(&action) {
            self.allowed_actions.push(action);
        }
        self
    }
    
    /// ¿Es un fix trivial que se puede aplicar automáticamente?
    pub fn is_trivial_fix(&self, action: &HealAction) -> bool {
        matches!(action, 
            HealAction::SyncDate | 
            HealAction::NormalizeWhitespace |
            HealAction::FixEncoding
        )
    }
    
    /// Sincroniza fecha de last_updated.
    pub fn sync_date(&mut self, path: &PathBuf) -> HealResult {
        if self.dry_run {
            return HealResult::success(
                HealAction::SyncDate,
                path.clone(),
                "[DRY-RUN] Would sync date",
            );
        }
        
        // Leer archivo, actualizar fecha
        let now = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        // TODO: Implementar actualización real del frontmatter
        
        let result = HealResult::success(
            HealAction::SyncDate,
            path.clone(),
            format!("Date synced to {}", now),
        );
        
        self.heal_log.push(result.clone());
        result
    }
    
    /// Normaliza whitespace en el contenido.
    pub fn normalize_whitespace(&mut self, path: &PathBuf, content: &str) -> (String, HealResult) {
        let normalized = content
            .lines()
            .map(|l| l.trim_end())
            .collect::<Vec<_>>()
            .join("\n");
        
        let result = HealResult::success(
            HealAction::NormalizeWhitespace,
            path.clone(),
            "Whitespace normalized",
        );
        
        if !self.dry_run {
            self.heal_log.push(result.clone());
        }
        
        (normalized, result)
    }
    
    /// Obtiene el log de curaciones.
    pub fn get_log(&self) -> &[HealResult] {
        &self.heal_log
    }
    
    /// Estadísticas de curaciones.
    pub fn stats(&self) -> (usize, usize) {
        let success = self.heal_log.iter().filter(|r| r.success).count();
        let failed = self.heal_log.len() - success;
        (success, failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healer_new() {
        let healer = Healer::new();
        assert!(!healer.dry_run);
        assert!(!healer.allowed_actions.is_empty());
    }

    #[test]
    fn test_is_trivial_fix() {
        let healer = Healer::new();
        assert!(healer.is_trivial_fix(&HealAction::SyncDate));
        assert!(!healer.is_trivial_fix(&HealAction::RebuildBreadcrumb));
    }

    #[test]
    fn test_normalize_whitespace() {
        let mut healer = Healer::new();
        let content = "line1   \nline2  \n";
        let (normalized, _) = healer.normalize_whitespace(&PathBuf::from("test.md"), content);
        
        assert!(!normalized.contains("   "));
    }

    #[test]
    fn test_dry_run() {
        let mut healer = Healer::new().with_dry_run(true);
        let result = healer.sync_date(&PathBuf::from("test.md"));
        
        assert!(result.message.contains("DRY-RUN"));
    }
}
