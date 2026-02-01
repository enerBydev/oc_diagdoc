//! Trait Repairable - Para reparación automática de documentos.

use std::path::PathBuf;

/// Tipo de reparación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairType {
    /// Reparación trivial (automática).
    Trivial,
    /// Reparación menor (sugiere al usuario).
    Minor,
    /// Reparación mayor (requiere confirmación).
    Major,
    /// Reparación crítica (puede perder datos).
    Critical,
}

impl RepairType {
    pub fn is_auto_safe(&self) -> bool {
        matches!(self, RepairType::Trivial)
    }
    
    pub fn requires_confirmation(&self) -> bool {
        matches!(self, RepairType::Major | RepairType::Critical)
    }
}

/// Acción de reparación.
#[derive(Debug, Clone)]
pub struct RepairAction {
    pub target: PathBuf,
    pub repair_type: RepairType,
    pub description: String,
    pub apply: Box<fn(&PathBuf) -> Result<(), String>>,
}

impl RepairAction {
    pub fn new(target: PathBuf, repair_type: RepairType, description: impl Into<String>) -> Self {
        Self {
            target,
            repair_type,
            description: description.into(),
            apply: Box::new(|_| Ok(())),
        }
    }
    
    pub fn trivial(target: PathBuf, description: impl Into<String>) -> Self {
        Self::new(target, RepairType::Trivial, description)
    }
    
    pub fn minor(target: PathBuf, description: impl Into<String>) -> Self {
        Self::new(target, RepairType::Minor, description)
    }
}

/// Trait para elementos que pueden ser reparados.
pub trait Repairable {
    /// Detecta problemas reparables.
    fn detect_issues(&self) -> Vec<RepairAction>;
    
    /// ¿Tiene problemas reparables?
    fn has_issues(&self) -> bool {
        !self.detect_issues().is_empty()
    }
    
    /// Filtra solo reparaciones automáticas seguras.
    fn auto_repairable(&self) -> Vec<RepairAction> {
        self.detect_issues()
            .into_iter()
            .filter(|r| r.repair_type.is_auto_safe())
            .collect()
    }
    
    /// Cuenta problemas por tipo.
    fn issue_summary(&self) -> (usize, usize, usize, usize) {
        let issues = self.detect_issues();
        let trivial = issues.iter().filter(|r| r.repair_type == RepairType::Trivial).count();
        let minor = issues.iter().filter(|r| r.repair_type == RepairType::Minor).count();
        let major = issues.iter().filter(|r| r.repair_type == RepairType::Major).count();
        let critical = issues.iter().filter(|r| r.repair_type == RepairType::Critical).count();
        (trivial, minor, major, critical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_repair_type_auto_safe() {
        assert!(RepairType::Trivial.is_auto_safe());
        assert!(!RepairType::Major.is_auto_safe());
    }
    
    #[test]
    fn test_requires_confirmation() {
        assert!(!RepairType::Minor.requires_confirmation());
        assert!(RepairType::Critical.requires_confirmation());
    }
    
    #[test]
    fn test_repair_action_trivial() {
        let action = RepairAction::trivial(PathBuf::from("test.md"), "Fix whitespace");
        assert!(action.repair_type.is_auto_safe());
    }
}
