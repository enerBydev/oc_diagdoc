//! Trait para validación uniforme de estructuras.

use crate::errors::OcError;

// ═══════════════════════════════════════════════════════════════════════════
// TRAIT VALIDATABLE
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para objetos que pueden validarse.
pub trait Validatable {
    /// Valida el objeto, retorna error si hay problemas.
    fn validate(&self) -> Result<(), OcError>;

    /// Valida y retorna lista de todos los errores encontrados.
    fn validate_all(&self) -> Vec<OcError> {
        match self.validate() {
            Ok(()) => Vec::new(),
            Err(e) => vec![e],
        }
    }

    /// ¿Es válido?
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Valida o panic con mensaje.
    fn expect_valid(&self, msg: &str) {
        if let Err(e) = self.validate() {
            panic!("{}: {}", msg, e);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// VALIDATABLE COLLECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Extensión para colecciones de Validatable.
pub trait ValidatableCollection {
    type Item: Validatable;
    
    /// Valida todos los elementos.
    fn validate_all_items(&self) -> Vec<OcError>;
    
    /// ¿Todos válidos?
    fn all_valid(&self) -> bool;
    
    /// Cuenta los válidos.
    fn count_valid(&self) -> usize;
    
    /// Cuenta inválidos.
    fn count_invalid(&self) -> usize;
}

impl<T: Validatable> ValidatableCollection for Vec<T> {
    type Item = T;
    
    fn validate_all_items(&self) -> Vec<OcError> {
        self.iter()
            .flat_map(|item| item.validate_all())
            .collect()
    }
    
    fn all_valid(&self) -> bool {
        self.iter().all(|item| item.is_valid())
    }
    
    fn count_valid(&self) -> usize {
        self.iter().filter(|item| item.is_valid()).count()
    }
    
    fn count_invalid(&self) -> usize {
        self.iter().filter(|item| !item.is_valid()).count()
    }
}

impl<T: Validatable> ValidatableCollection for [T] {
    type Item = T;
    
    fn validate_all_items(&self) -> Vec<OcError> {
        self.iter()
            .flat_map(|item| item.validate_all())
            .collect()
    }
    
    fn all_valid(&self) -> bool {
        self.iter().all(|item| item.is_valid())
    }
    
    fn count_valid(&self) -> usize {
        self.iter().filter(|item| item.is_valid()).count()
    }
    
    fn count_invalid(&self) -> usize {
        self.iter().filter(|item| !item.is_valid()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestItem { valid: bool }
    
    impl Validatable for TestItem {
        fn validate(&self) -> Result<(), OcError> {
            if self.valid {
                Ok(())
            } else {
                Err(OcError::Validation { message: "invalid".to_string() })
            }
        }
    }

    #[test]
    fn test_validatable_is_valid() {
        let valid = TestItem { valid: true };
        let invalid = TestItem { valid: false };
        
        assert!(valid.is_valid());
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_validatable_collection() {
        let items = vec![
            TestItem { valid: true },
            TestItem { valid: false },
            TestItem { valid: true },
        ];
        
        assert!(!items.all_valid());
        assert_eq!(items.count_valid(), 2);
        assert_eq!(items.count_invalid(), 1);
    }

    #[test]
    fn test_validate_all() {
        let item = TestItem { valid: false };
        let errors = item.validate_all();
        assert_eq!(errors.len(), 1);
    }
}
