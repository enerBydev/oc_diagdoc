//! Trait Diffable - Para comparación de versiones de documentos.

/// Diferencia entre dos versiones.
#[derive(Debug, Clone)]
pub struct Diff {
    pub added_lines: Vec<String>,
    pub removed_lines: Vec<String>,
    pub modified_count: usize,
}

impl Diff {
    pub fn new() -> Self {
        Self {
            added_lines: Vec::new(),
            removed_lines: Vec::new(),
            modified_count: 0,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.added_lines.is_empty() && self.removed_lines.is_empty()
    }
    
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} ~{}",
            self.added_lines.len(),
            self.removed_lines.len(),
            self.modified_count
        )
    }
}

impl Default for Diff {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait para elementos que pueden ser comparados.
pub trait Diffable {
    /// Contenido como string para comparación.
    fn content(&self) -> &str;
    
    /// Calcula diferencia respecto a otra versión.
    fn diff(&self, other: &Self) -> Diff {
        let self_lines: Vec<&str> = self.content().lines().collect();
        let other_lines: Vec<&str> = other.content().lines().collect();
        
        let mut diff = Diff::new();
        
        // Simple line-by-line diff
        for line in &self_lines {
            if !other_lines.contains(line) {
                diff.removed_lines.push(line.to_string());
            }
        }
        
        for line in &other_lines {
            if !self_lines.contains(line) {
                diff.added_lines.push(line.to_string());
            }
        }
        
        diff.modified_count = diff.added_lines.len().min(diff.removed_lines.len());
        diff
    }
    
    /// ¿Son idénticos?
    fn is_identical(&self, other: &Self) -> bool {
        self.content() == other.content()
    }
    
    /// Porcentaje de similitud (0.0 - 100.0).
    fn similarity(&self, other: &Self) -> f64 {
        if self.is_identical(other) {
            return 100.0;
        }
        
        let diff = self.diff(other);
        let total_lines = self.content().lines().count().max(1) as f64;
        let changed = (diff.added_lines.len() + diff.removed_lines.len()) as f64;
        
        ((total_lines - changed / 2.0) / total_lines * 100.0).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TextContent(String);
    
    impl Diffable for TextContent {
        fn content(&self) -> &str { &self.0 }
    }
    
    #[test]
    fn test_identical() {
        let a = TextContent("hello\nworld".to_string());
        let b = TextContent("hello\nworld".to_string());
        assert!(a.is_identical(&b));
    }
    
    #[test]
    fn test_diff() {
        let a = TextContent("line1\nline2".to_string());
        let b = TextContent("line1\nline3".to_string());
        let diff = a.diff(&b);
        
        assert_eq!(diff.removed_lines.len(), 1);
        assert_eq!(diff.added_lines.len(), 1);
    }
}
