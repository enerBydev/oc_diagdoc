//! Trait para reparación automática.

use std::path::PathBuf;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// FIX RESULT
// ═══════════════════════════════════════════════════════════════════════════

/// Resultado de aplicar un fix.
#[derive(Debug, Clone)]
pub enum FixResult {
    /// Fix aplicado exitosamente.
    Applied { description: String },
    /// No se pudo aplicar.
    Failed { reason: String },
    /// No era necesario.
    NotNeeded,
    /// Requiere confirmación del usuario.
    NeedsConfirmation { description: String },
}

impl FixResult {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Applied { .. } | Self::NotNeeded)
    }
    
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FIX
// ═══════════════════════════════════════════════════════════════════════════

/// Representa un fix propuesto.
#[derive(Debug, Clone)]
pub struct Fix {
    /// Código del fix.
    pub code: String,
    /// Descripción del fix.
    pub description: String,
    /// Archivo a modificar.
    pub file: Option<PathBuf>,
    /// ¿Es automático o requiere confirmación?
    pub auto_apply: bool,
    /// Contenido antes del fix (para preview).
    pub before: Option<String>,
    /// Contenido después del fix (para preview).
    pub after: Option<String>,
}

impl Fix {
    pub fn new(code: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            file: None,
            auto_apply: true,
            before: None,
            after: None,
        }
    }
    
    pub fn with_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = Some(file.into());
        self
    }
    
    pub fn manual(mut self) -> Self {
        self.auto_apply = false;
        self
    }
    
    pub fn with_preview(mut self, before: impl Into<String>, after: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self.after = Some(after.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT FIXABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos que pueden ser reparados automáticamente.
pub trait Fixable {
    /// Sugiere fixes disponibles.
    fn suggest_fixes(&self) -> Vec<Fix>;
    
    /// Aplica un fix específico.
    fn apply_fix(&mut self, fix: &Fix) -> OcResult<FixResult>;
    
    /// Aplica todos los fixes automáticos.
    fn apply_all_auto_fixes(&mut self) -> Vec<FixResult> {
        let fixes: Vec<Fix> = self.suggest_fixes()
            .into_iter()
            .filter(|f| f.auto_apply)
            .collect();
        
        fixes.iter()
            .map(|fix| self.apply_fix(fix).unwrap_or(FixResult::Failed {
                reason: "Failed to apply".to_string(),
            }))
            .collect()
    }
    
    /// ¿Tiene fixes disponibles?
    fn has_fixes(&self) -> bool {
        !self.suggest_fixes().is_empty()
    }
    
    /// Cuenta fixes automáticos disponibles.
    fn auto_fix_count(&self) -> usize {
        self.suggest_fixes().iter().filter(|f| f.auto_apply).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_new() {
        let fix = Fix::new("F001", "Fix description")
            .with_file("test.md");
        
        assert_eq!(fix.code, "F001");
        assert!(fix.auto_apply);
    }

    #[test]
    fn test_fix_manual() {
        let fix = Fix::new("F001", "Manual fix").manual();
        assert!(!fix.auto_apply);
    }

    #[test]
    fn test_fix_result() {
        let success = FixResult::Applied { description: "Done".to_string() };
        let failure = FixResult::Failed { reason: "Error".to_string() };
        
        assert!(success.is_success());
        assert!(failure.is_failure());
    }
}
