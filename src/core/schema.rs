//! Validador de esquema para frontmatter YAML.
//!
//! Proporciona:
//! - Definición de esquemas con campos requeridos/opcionales
//! - Validación de documentos contra esquemas
//! - Sugerencias de corrección automáticas

use crate::core::yaml::YamlFrontmatter;
use crate::errors::{OcError, OcResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Tipo de campo esperado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Integer,
    Boolean,
    Date,
    Array,
    /// Enum con valores permitidos.
    Enum(Vec<String>),
}

impl Default for FieldType {
    fn default() -> Self {
        Self::String
    }
}

/// Especificación de un campo del esquema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    /// Nombre del campo.
    pub name: String,
    /// Tipo esperado.
    #[serde(default)]
    pub field_type: FieldType,
    /// ¿Es requerido?
    #[serde(default)]
    pub required: bool,
    /// Valor por defecto si no está presente.
    pub default_value: Option<String>,
    /// Descripción del campo.
    pub description: Option<String>,
    /// Patrón regex de validación.
    pub pattern: Option<String>,
}

impl FieldSpec {
    pub fn required(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            field_type,
            required: true,
            default_value: None,
            description: None,
            pattern: None,
        }
    }

    pub fn optional(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            field_type,
            required: false,
            default_value: None,
            description: None,
            pattern: None,
        }
    }

    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }
}

/// Definición completa de un esquema.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDefinition {
    /// Nombre del esquema.
    pub name: String,
    /// Versión del esquema.
    pub version: String,
    /// Campos del esquema.
    pub fields: Vec<FieldSpec>,
}

impl SchemaDefinition {
    /// Crea un nuevo esquema.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "1.0".to_string(),
            fields: Vec::new(),
        }
    }

    /// Agrega un campo.
    pub fn add_field(mut self, field: FieldSpec) -> Self {
        self.fields.push(field);
        self
    }

    /// Campos requeridos.
    pub fn required_fields(&self) -> Vec<&FieldSpec> {
        self.fields.iter().filter(|f| f.required).collect()
    }

    /// Campos opcionales.
    pub fn optional_fields(&self) -> Vec<&FieldSpec> {
        self.fields.iter().filter(|f| !f.required).collect()
    }

    /// Crea el esquema por defecto de oc_diagdoc.
    pub fn default_oc_schema() -> Self {
        Self::new("oc_diagdoc")
            .add_field(FieldSpec::required("id", FieldType::String))
            .add_field(FieldSpec::required("title", FieldType::String))
            .add_field(FieldSpec::optional("parent", FieldType::String))
            .add_field(FieldSpec::optional("breadcrumb", FieldType::String))
            .add_field(
                FieldSpec::optional(
                    "status",
                    FieldType::Enum(vec![
                        "draft".into(),
                        "active".into(),
                        "deprecated".into(),
                        "archived".into(),
                        "reviewed".into(),
                    ]),
                )
                .with_default("draft"),
            )
            .add_field(FieldSpec::optional("created", FieldType::Date))
            .add_field(FieldSpec::optional("last_updated", FieldType::Date))
            .add_field(FieldSpec::optional("author", FieldType::String))
            .add_field(FieldSpec::optional("domain", FieldType::String))
            .add_field(FieldSpec::optional("tags", FieldType::Array))
            .add_field(FieldSpec::optional("priority", FieldType::String))
    }
}

/// Violación de esquema detectada.
#[derive(Debug, Clone)]
pub struct SchemaViolation {
    /// Campo con problemas.
    pub field: String,
    /// Tipo de violación.
    pub violation_type: ViolationType,
    /// Mensaje descriptivo.
    pub message: String,
    /// Sugerencia de corrección.
    pub suggestion: Option<String>,
}

/// Tipos de violaciones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    /// Campo requerido faltante.
    MissingRequired,
    /// Tipo incorrecto.
    WrongType,
    /// Valor no permitido (enum).
    InvalidValue,
    /// Patrón no coincide.
    PatternMismatch,
}

/// Resultado de validación.
#[derive(Debug, Default)]
pub struct ValidationResult {
    /// ¿Es válido?
    pub is_valid: bool,
    /// Violaciones encontradas.
    pub violations: Vec<SchemaViolation>,
    /// Campos que tienen valores por defecto aplicables.
    pub defaults_applicable: Vec<(String, String)>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            ..Default::default()
        }
    }

    pub fn add_violation(&mut self, violation: SchemaViolation) {
        self.is_valid = false;
        self.violations.push(violation);
    }
}

/// Valida un frontmatter contra un esquema.
pub fn validate_frontmatter(
    frontmatter: &YamlFrontmatter,
    schema: &SchemaDefinition,
) -> ValidationResult {
    let mut result = ValidationResult::valid();

    // Convertir frontmatter a mapa para inspección
    let fm_map = frontmatter_to_map(frontmatter);

    for field_spec in &schema.fields {
        let field_value = fm_map.get(&field_spec.name);

        // Verificar campos requeridos
        if field_spec.required && field_value.is_none() {
            result.add_violation(SchemaViolation {
                field: field_spec.name.clone(),
                violation_type: ViolationType::MissingRequired,
                message: format!("Campo requerido '{}' no encontrado", field_spec.name),
                suggestion: field_spec
                    .default_value
                    .as_ref()
                    .map(|d| format!("Agregar: {}: {}", field_spec.name, d)),
            });
            continue;
        }

        // Verificar tipo si existe valor
        if let Some(value) = field_value {
            if !validate_type(value, &field_spec.field_type) {
                result.add_violation(SchemaViolation {
                    field: field_spec.name.clone(),
                    violation_type: ViolationType::WrongType,
                    message: format!(
                        "Campo '{}' tiene tipo incorrecto. Esperado: {:?}",
                        field_spec.name, field_spec.field_type
                    ),
                    suggestion: None,
                });
            }

            // Verificar enum
            if let FieldType::Enum(allowed) = &field_spec.field_type {
                if !allowed.contains(value) {
                    result.add_violation(SchemaViolation {
                        field: field_spec.name.clone(),
                        violation_type: ViolationType::InvalidValue,
                        message: format!(
                            "Valor '{}' no permitido para '{}'. Valores válidos: {:?}",
                            value, field_spec.name, allowed
                        ),
                        suggestion: Some(format!("Usar uno de: {}", allowed.join(", "))),
                    });
                }
            }
        } else if let Some(default) = &field_spec.default_value {
            result
                .defaults_applicable
                .push((field_spec.name.clone(), default.clone()));
        }
    }

    result
}

/// Convierte frontmatter a mapa de strings (solo valores no vacíos).
fn frontmatter_to_map(fm: &YamlFrontmatter) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Solo incluir si no está vacío
    if !fm.id.is_empty() {
        map.insert("id".to_string(), fm.id.clone());
    }
    if !fm.title.is_empty() {
        map.insert("title".to_string(), fm.title.clone());
    }

    if let Some(p) = &fm.parent {
        map.insert("parent".to_string(), p.clone());
    }
    if !fm.breadcrumb.is_empty() {
        map.insert("breadcrumb".to_string(), fm.breadcrumb.clone());
    }
    if !fm.status.is_empty() {
        map.insert("status".to_string(), fm.status.clone());
    }
    if let Some(c) = &fm.created {
        map.insert("created".to_string(), c.clone());
    }
    if let Some(u) = &fm.last_updated {
        map.insert("last_updated".to_string(), u.clone());
    }
    if let Some(a) = &fm.author {
        map.insert("author".to_string(), a.clone());
    }
    if let Some(d) = &fm.domain {
        map.insert("domain".to_string(), d.clone());
    }
    if let Some(t) = &fm.doc_type {
        map.insert("doc_type".to_string(), t.clone());
    }
    if let Some(p) = &fm.priority {
        map.insert("priority".to_string(), p.clone());
    }

    map
}

/// Valida que un valor sea del tipo esperado.
fn validate_type(value: &str, expected: &FieldType) -> bool {
    match expected {
        FieldType::String => true,
        FieldType::Integer => value.parse::<i64>().is_ok(),
        FieldType::Boolean => value == "true" || value == "false",
        FieldType::Date => {
            // Formato básico YYYY-MM-DD
            value.len() >= 10 && value.chars().nth(4) == Some('-')
        }
        FieldType::Array => true,   // No validamos contenido aquí
        FieldType::Enum(_) => true, // Validado aparte
    }
}

/// Genera sugerencias de corrección.
pub fn suggest_fixes(violations: &[SchemaViolation]) -> Vec<String> {
    violations
        .iter()
        .filter_map(|v| v.suggestion.clone())
        .collect()
}

/// Carga esquema desde archivo YAML.
pub fn load_schema(path: impl AsRef<Path>) -> OcResult<SchemaDefinition> {
    let content = std::fs::read_to_string(path.as_ref()).map_err(|e| OcError::FileRead {
        path: path.as_ref().to_path_buf(),
        source: e,
    })?;

    serde_yaml::from_str(&content).map_err(|e| OcError::YamlParse {
        path: path.as_ref().to_path_buf(),
        message: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = SchemaDefinition::default_oc_schema();
        assert_eq!(schema.required_fields().len(), 2); // id, title
        assert!(schema.optional_fields().len() > 5);
    }

    #[test]
    fn test_valid_frontmatter() {
        let schema = SchemaDefinition::default_oc_schema();
        let fm = YamlFrontmatter {
            id: "1.1".to_string(),
            title: "Test Document".to_string(),
            status: "active".to_string(),
            ..Default::default()
        };

        let result = validate_frontmatter(&fm, &schema);
        assert!(result.is_valid);
    }

    #[test]
    fn test_missing_required() {
        let schema = SchemaDefinition::default_oc_schema();
        let fm = YamlFrontmatter {
            id: String::new(), // Empty = missing
            title: "Test".to_string(),
            ..Default::default()
        };

        let result = validate_frontmatter(&fm, &schema);
        assert!(!result.is_valid);
        assert!(result.violations.iter().any(|v| v.field == "id"));
    }

    #[test]
    fn test_invalid_enum() {
        let schema = SchemaDefinition::default_oc_schema();
        let fm = YamlFrontmatter {
            id: "1".to_string(),
            title: "Test".to_string(),
            status: "invalid_status".to_string(),
            ..Default::default()
        };

        let result = validate_frontmatter(&fm, &schema);
        assert!(!result.is_valid);
        assert!(result
            .violations
            .iter()
            .any(|v| v.violation_type == ViolationType::InvalidValue));
    }

    #[test]
    fn test_defaults_applicable() {
        let schema = SchemaDefinition::new("test")
            .add_field(FieldSpec::required("id", FieldType::String))
            .add_field(FieldSpec::optional("status", FieldType::String).with_default("draft"));

        let fm = YamlFrontmatter {
            id: "1".to_string(),
            title: "Test".to_string(),
            status: String::new(),
            ..Default::default()
        };

        let result = validate_frontmatter(&fm, &schema);
        assert!(!result.defaults_applicable.is_empty());
    }

    #[test]
    fn test_suggest_fixes() {
        let violations = vec![SchemaViolation {
            field: "status".to_string(),
            violation_type: ViolationType::InvalidValue,
            message: "Invalid".to_string(),
            suggestion: Some("Use 'draft'".to_string()),
        }];

        let fixes = suggest_fixes(&violations);
        assert_eq!(fixes.len(), 1);
        assert!(fixes[0].contains("draft"));
    }
}
