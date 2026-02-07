//! RFC-02: Fix Router - Enrutador inteligente de correcciones
//!
//! Mapea anomalías detectadas a comandos de corrección sugeridos.

use std::collections::HashMap;

/// Tipo de anomalía detectada.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AnomalyType {
    // Errores de Fechas
    DateDrift,
    HashMismatch,
    
    // Errores de Enlaces
    BrokenLink,
    CaseSensitiveLink,
    
    // Errores de YAML
    MissingYamlFrontmatter,
    InvalidType,
    InvalidStatus,
    
    // Errores de Tablas
    NietosCountMismatch,
    ChildrenCountMismatch,
    
    // Errores de Lint
    TrailingWhitespace,
    MissingFinalNewline,
    LineTooLong,
    CodeBlockNoLanguage,
    DuplicateHeader,
    TableNoHeader,
    ImageNoAlt,
    
    // Errores Estructurales
    OrphanDocument,
    BreadcrumbInconsistent,
}

/// Severidad de la anomalía.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl FixSeverity {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Critical => "🔴",
            Self::High => "🟠",
            Self::Medium => "🟡",
            Self::Low => "🔵",
        }
    }
}

/// Comando sugerido para corrección.
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub command: String,
    pub description: String,
    pub auto_safe: bool,
    pub severity: FixSeverity,
}

/// Router que mapea anomalías a comandos de corrección.
pub struct FixRouter {
    routes: HashMap<AnomalyType, FixSuggestion>,
}

impl FixRouter {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        
        // Fechas/Hashes
        routes.insert(AnomalyType::DateDrift, FixSuggestion {
            command: "sync --dates-only --force".into(),
            description: "Sincroniza fechas YAML con mtime".into(),
            auto_safe: true,
            severity: FixSeverity::Medium,
        });
        
        routes.insert(AnomalyType::HashMismatch, FixSuggestion {
            command: "sync --hashes-only".into(),
            description: "Recalcula content_hash".into(),
            auto_safe: true,
            severity: FixSeverity::High,
        });
        
        // Enlaces
        routes.insert(AnomalyType::BrokenLink, FixSuggestion {
            command: "links --fix-broken".into(),
            description: "Repara enlaces rotos".into(),
            auto_safe: false,
            severity: FixSeverity::Critical,
        });
        
        routes.insert(AnomalyType::CaseSensitiveLink, FixSuggestion {
            command: "links --fix-case".into(),
            description: "Corrige mayúsculas/minúsculas".into(),
            auto_safe: true,
            severity: FixSeverity::High,
        });
        
        // Tablas
        routes.insert(AnomalyType::NietosCountMismatch, FixSuggestion {
            command: "sync --fix-all".into(),
            description: "Sincroniza conteos de Nietos".into(),
            auto_safe: true,
            severity: FixSeverity::High,
        });
        
        routes.insert(AnomalyType::ChildrenCountMismatch, FixSuggestion {
            command: "sync --children".into(),
            description: "Recalcula children_count".into(),
            auto_safe: true,
            severity: FixSeverity::High,
        });
        
        // Lint
        routes.insert(AnomalyType::TrailingWhitespace, FixSuggestion {
            command: "lint --fix".into(),
            description: "Elimina espacios trailing".into(),
            auto_safe: true,
            severity: FixSeverity::Low,
        });
        
        routes.insert(AnomalyType::MissingFinalNewline, FixSuggestion {
            command: "lint --fix".into(),
            description: "Agrega newline final".into(),
            auto_safe: true,
            severity: FixSeverity::Low,
        });
        
        routes.insert(AnomalyType::CodeBlockNoLanguage, FixSuggestion {
            command: "# MANUAL: Agregar lenguaje a ```".into(),
            description: "Especificar lenguaje del código".into(),
            auto_safe: false,
            severity: FixSeverity::Low,
        });
        
        // YAML
        routes.insert(AnomalyType::MissingYamlFrontmatter, FixSuggestion {
            command: "init --add-yaml".into(),
            description: "Agrega frontmatter YAML".into(),
            auto_safe: false,
            severity: FixSeverity::Critical,
        });
        
        routes.insert(AnomalyType::BreadcrumbInconsistent, FixSuggestion {
            command: "sync --breadcrumbs".into(),
            description: "Regenera breadcrumbs".into(),
            auto_safe: true,
            severity: FixSeverity::Medium,
        });
        
        Self { routes }
    }
    
    /// Obtiene la sugerencia para una anomalía.
    pub fn suggest(&self, anomaly: &AnomalyType) -> Option<&FixSuggestion> {
        self.routes.get(anomaly)
    }
    
    /// Genera comando completo con path.
    pub fn generate_command(&self, anomaly: &AnomalyType, target: &str) -> Option<String> {
        self.suggest(anomaly).map(|s| {
            if s.command.starts_with('#') {
                s.command.clone()
            } else {
                format!("oc_diagdoc {} --path {}", s.command, target)
            }
        })
    }
}

impl Default for FixRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Imprime resumen de fixes sugeridos (para CLI)
pub fn print_fix_summary(anomalies: &HashMap<AnomalyType, Vec<String>>) {
    if anomalies.is_empty() {
        return;
    }
    
    let router = FixRouter::new();
    
    println!();
    println!("╭─────────────────────────────────────────────────────────────╮");
    println!("│  📋 RESUMEN DE ANOMALÍAS DETECTADAS                         │");
    println!("├─────────────────────────────────────────────────────────────┤");
    
    for (anomaly_type, instances) in anomalies {
        let count = instances.len();
        if let Some(suggestion) = router.suggest(anomaly_type) {
            println!("│  {} {} errores: {:?}", suggestion.severity.icon(), count, anomaly_type);
            println!("│     └─ FIX: {}", suggestion.command);
        } else {
            println!("│  ⚪ {} errores: {:?}", count, anomaly_type);
        }
        println!("│");
    }
    
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!();
    println!("💡 Ejecuta los comandos sugeridos para corregir las anomalías.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fix_router_suggest() {
        let router = FixRouter::new();
        let suggestion = router.suggest(&AnomalyType::DateDrift);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().command.contains("sync"));
    }
    
    #[test]
    fn test_generate_command() {
        let router = FixRouter::new();
        let cmd = router.generate_command(&AnomalyType::HashMismatch, "Datos");
        assert!(cmd.is_some());
        assert!(cmd.unwrap().contains("--path Datos"));
    }
}
