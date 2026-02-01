//! Trait Hierarchical - Para estructuras jerárquicas de documentación.

use std::path::PathBuf;

/// Trait para elementos con jerarquía padre-hijo.
pub trait Hierarchical {
    /// ID del elemento (e.g., "1.2.3").
    fn id(&self) -> &str;
    
    /// ID del padre (Option porque root no tiene padre).
    fn parent_id(&self) -> Option<&str>;
    
    /// Profundidad en el árbol (root = 0).
    fn depth(&self) -> usize {
        self.id().matches('.').count()
    }
    
    /// ¿Es un nodo raíz?
    fn is_root(&self) -> bool {
        self.parent_id().is_none() || self.parent_id() == Some("0")
    }
    
    /// Ancestros (lista de IDs desde root hasta parent).
    fn ancestors(&self) -> Vec<String> {
        let id = self.id();
        let parts: Vec<&str> = id.split('.').collect();
        let mut ancestors = Vec::new();
        
        for i in 1..parts.len() {
            ancestors.push(parts[..i].join("."));
        }
        ancestors
    }
    
    /// Módulo (primer componente del ID).
    fn module(&self) -> String {
        self.id().split('.').next().unwrap_or("0").to_string()
    }
}

/// Implementación para PathBuf basado en nombre de archivo.
impl Hierarchical for PathBuf {
    fn id(&self) -> &str {
        self.file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split_whitespace().next())
            .unwrap_or("0")
    }
    
    fn parent_id(&self) -> Option<&str> {
        let id = self.id();
        if !id.contains('.') {
            return None;
        }
        // Return everything except last component
        id.rfind('.').map(|i| &id[..i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_depth() {
        let path = PathBuf::from("1.2.3 Documento.md");
        assert_eq!(path.depth(), 2);
    }
    
    #[test]
    fn test_ancestors() {
        let path = PathBuf::from("1.2.3.4 Documento.md");
        let ancestors = path.ancestors();
        assert_eq!(ancestors, vec!["1", "1.2", "1.2.3"]);
    }
    
    #[test]
    fn test_module() {
        let path = PathBuf::from("3.5.2 Documento.md");
        assert_eq!(path.module(), "3");
    }
}
