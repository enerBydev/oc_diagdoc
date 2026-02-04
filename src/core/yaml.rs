//! YAML Frontmatter parser para documentos markdown.
//!
//! Proporciona funcionalidad para:
//! - Parsear frontmatter YAML de documentos markdown
//! - Extraer el body del documento
//! - Actualizar campos individuales preservando formato

use crate::errors::{OcError, OcResult};
use crate::types::{Breadcrumb, DocumentId, DocumentStatus, OcDate};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Delimitador de frontmatter YAML.
pub const FRONTMATTER_DELIMITER: &str = "---";

/// Frontmatter YAML de un documento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlFrontmatter {
    // ═══════════════════════════════════════════════════════════════
    // CAMPOS REQUERIDOS
    // ═══════════════════════════════════════════════════════════════
    /// ID jerárquico del documento (ej: "3.1.2").
    pub id: String,

    /// Título del documento.
    pub title: String,

    /// ID del padre (vacío para master).
    #[serde(default)]
    pub parent: Option<String>,

    /// Breadcrumb jerárquico.
    #[serde(default)]
    pub breadcrumb: String,

    /// Estado del documento.
    #[serde(default)]
    pub status: String,

    // ═══════════════════════════════════════════════════════════════
    // CAMPOS SEMI-REQUERIDOS
    // ═══════════════════════════════════════════════════════════════
    /// Tipo de documento.
    #[serde(default, alias = "type")]
    pub doc_type: Option<String>,

    /// Fecha de creación.
    #[serde(default)]
    pub created: Option<String>,

    /// Fecha de última actualización.
    #[serde(default)]
    pub last_updated: Option<String>,

    /// Autor.
    #[serde(default)]
    pub author: Option<String>,

    // ═══════════════════════════════════════════════════════════════
    // CAMPOS OPCIONALES
    // ═══════════════════════════════════════════════════════════════
    /// Dominio funcional.
    #[serde(default)]
    pub domain: Option<String>,

    /// Actores involucrados.
    #[serde(default)]
    pub actors: Option<Vec<String>>,

    /// Tags/etiquetas.
    #[serde(default)]
    pub tags: Option<Vec<String>>,

    /// Prioridad.
    #[serde(default)]
    pub priority: Option<String>,

    /// Descripción corta.
    #[serde(default)]
    pub description: Option<String>,

    /// Número de hijos directos.
    #[serde(default)]
    pub children_count: Option<usize>,

    /// Número total de descendientes.
    #[serde(default)]
    pub descendants_count: Option<usize>,

    /// Hash de contenido.
    #[serde(default)]
    pub content_hash: Option<String>,

    /// Referencias/enlaces internos.
    #[serde(default)]
    pub references: Option<Vec<String>>,

    /// Versión del documento.
    #[serde(default)]
    pub version: Option<String>,
}

impl Default for YamlFrontmatter {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            parent: None,
            breadcrumb: String::new(),
            status: "draft".to_string(),
            doc_type: None,
            created: None,
            last_updated: None,
            author: None,
            domain: None,
            actors: None,
            tags: None,
            priority: None,
            description: None,
            children_count: None,
            descendants_count: None,
            content_hash: None,
            references: None,
            version: None,
        }
    }
}

impl YamlFrontmatter {
    /// Parsea el ID como DocumentId.
    pub fn document_id(&self) -> OcResult<DocumentId> {
        self.id.parse()
    }

    /// Parsea el status como DocumentStatus.
    pub fn document_status(&self) -> OcResult<DocumentStatus> {
        self.status.parse()
    }

    /// Parsea el breadcrumb como Breadcrumb.
    pub fn parsed_breadcrumb(&self) -> Breadcrumb {
        Breadcrumb::parse(&self.breadcrumb)
    }

    /// Parsea la fecha de creación.
    pub fn created_date(&self) -> Option<OcResult<OcDate>> {
        self.created.as_ref().map(|s| s.parse())
    }

    /// Parsea la fecha de última actualización.
    pub fn last_updated_date(&self) -> Option<OcResult<OcDate>> {
        self.last_updated.as_ref().map(|s| s.parse())
    }

    /// ¿Es el documento master (ID = "0")?
    pub fn is_master(&self) -> bool {
        self.id == "0"
    }

    /// Valida campos requeridos.
    pub fn validate(&self) -> OcResult<()> {
        if self.id.is_empty() {
            return Err(OcError::MissingField {
                field: "id".to_string(),
                path: std::path::PathBuf::new(),
            });
        }
        if self.title.is_empty() {
            return Err(OcError::MissingField {
                field: "title".to_string(),
                path: std::path::PathBuf::new(),
            });
        }
        // Validar ID parseando
        let _id: DocumentId = self.id.parse()?;
        Ok(())
    }

    /// Crea un builder para construir YamlFrontmatter de forma fluida.
    pub fn builder() -> YamlFrontmatterBuilder {
        YamlFrontmatterBuilder::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R12: YamlFrontmatterBuilder - Patrón Builder para construcción fluida
// ═══════════════════════════════════════════════════════════════════════════

/// Builder para construir YamlFrontmatter de forma fluida.
#[derive(Debug, Default)]
pub struct YamlFrontmatterBuilder {
    id: Option<String>,
    title: Option<String>,
    parent: Option<String>,
    breadcrumb: Option<String>,
    status: Option<String>,
    doc_type: Option<String>,
    created: Option<String>,
    last_updated: Option<String>,
    author: Option<String>,
    domain: Option<String>,
    tags: Option<Vec<String>>,
    priority: Option<String>,
    description: Option<String>,
}

impl YamlFrontmatterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn breadcrumb(mut self, bc: impl Into<String>) -> Self {
        self.breadcrumb = Some(bc.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn doc_type(mut self, t: impl Into<String>) -> Self {
        self.doc_type = Some(t.into());
        self
    }

    pub fn created(mut self, date: impl Into<String>) -> Self {
        self.created = Some(date.into());
        self
    }

    pub fn last_updated(mut self, date: impl Into<String>) -> Self {
        self.last_updated = Some(date.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Construye el YamlFrontmatter, validando campos requeridos.
    pub fn build(self) -> OcResult<YamlFrontmatter> {
        let fm = YamlFrontmatter {
            id: self.id.ok_or_else(|| OcError::MissingField {
                field: "id".to_string(),
                path: std::path::PathBuf::new(),
            })?,
            title: self.title.ok_or_else(|| OcError::MissingField {
                field: "title".to_string(),
                path: std::path::PathBuf::new(),
            })?,
            parent: self.parent,
            breadcrumb: self.breadcrumb.unwrap_or_default(),
            status: self.status.unwrap_or_else(|| "draft".to_string()),
            doc_type: self.doc_type,
            created: self.created,
            last_updated: self.last_updated,
            author: self.author,
            domain: self.domain,
            actors: None,
            tags: self.tags,
            priority: self.priority,
            description: self.description,
            children_count: None,
            descendants_count: None,
            content_hash: None,
            references: None,
            version: None,
        };

        fm.validate()?;
        Ok(fm)
    }

    /// Construye sin validar (para casos especiales).
    pub fn build_unchecked(self) -> YamlFrontmatter {
        YamlFrontmatter {
            id: self.id.unwrap_or_default(),
            title: self.title.unwrap_or_default(),
            parent: self.parent,
            breadcrumb: self.breadcrumb.unwrap_or_default(),
            status: self.status.unwrap_or_else(|| "draft".to_string()),
            doc_type: self.doc_type,
            created: self.created,
            last_updated: self.last_updated,
            author: self.author,
            domain: self.domain,
            actors: None,
            tags: self.tags,
            priority: self.priority,
            description: self.description,
            children_count: None,
            descendants_count: None,
            content_hash: None,
            references: None,
            version: None,
        }
    }
}

/// Resultado de parsear un documento markdown.
#[derive(Debug)]
pub struct ParsedDocument {
    /// Frontmatter parseado.
    pub frontmatter: YamlFrontmatter,
    /// Contenido YAML raw.
    pub yaml_raw: String,
    /// Body del documento (después del frontmatter).
    pub body: String,
}

/// Parsea el frontmatter YAML de un contenido markdown.
pub fn parse_frontmatter(content: &str) -> OcResult<ParsedDocument> {
    let content = content.trim_start();

    // Verificar que empiece con ---
    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return Err(OcError::MissingFrontmatter(std::path::PathBuf::new()));
    }

    // Buscar el segundo ---
    let after_first = &content[3..];
    let end_pos = after_first
        .find(FRONTMATTER_DELIMITER)
        .ok_or_else(|| OcError::MissingFrontmatter(std::path::PathBuf::new()))?;

    let yaml_content = &after_first[..end_pos].trim();
    let body_start = 3 + end_pos + 3; // Primer --- + yaml + segundo ---
    let body = if body_start < content.len() {
        content[body_start..].trim_start().to_string()
    } else {
        String::new()
    };

    // Parsear YAML
    let frontmatter: YamlFrontmatter =
        serde_yaml::from_str(yaml_content).map_err(|e| OcError::YamlParse {
            path: std::path::PathBuf::new(),
            message: e.to_string(),
        })?;

    Ok(ParsedDocument {
        frontmatter,
        yaml_raw: yaml_content.to_string(),
        body,
    })
}

/// Parsea frontmatter de un archivo.
pub fn parse_frontmatter_from_file(path: impl AsRef<Path>) -> OcResult<ParsedDocument> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).map_err(|e| OcError::FileRead {
        path: path.to_path_buf(),
        source: e,
    })?;

    let result = parse_frontmatter(&content)?;

    // Actualizar paths en errores de validación si los hay
    if let Err(e) = result.frontmatter.validate() {
        return Err(match e {
            OcError::MissingField { field, .. } => OcError::MissingField {
                field,
                path: path.to_path_buf(),
            },
            other => other,
        });
    }

    Ok(result)
}

/// Extrae solo el body de un documento (sin frontmatter).
pub fn extract_body(content: &str) -> OcResult<String> {
    let parsed = parse_frontmatter(content)?;
    Ok(parsed.body)
}

/// Actualiza un campo específico en el frontmatter preservando formato.
pub fn update_field(content: &str, field: &str, value: &str) -> OcResult<String> {
    let content = content.trim_start();

    // Verificar frontmatter existe
    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return Err(OcError::MissingFrontmatter(std::path::PathBuf::new()));
    }

    // Buscar el campo en el YAML
    let pattern = format!("{}:", field);

    if let Some(field_pos) = content.find(&pattern) {
        // Encontrar el final de la línea
        let after_field = &content[field_pos..];
        let line_end = after_field.find('\n').unwrap_or(after_field.len());

        // Construir nueva línea
        let new_line = format!("{}: {}", field, value);

        // Reconstruir contenido
        let before = &content[..field_pos];
        let after = &content[field_pos + line_end..];

        Ok(format!("{}{}{}", before, new_line, after))
    } else {
        // Campo no existe, agregarlo antes del segundo ---
        add_field(content, field, value)
    }
}

/// Agrega un nuevo campo al frontmatter.
pub fn add_field(content: &str, field: &str, value: &str) -> OcResult<String> {
    let content = content.trim_start();

    // Verificar frontmatter existe
    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return Err(OcError::MissingFrontmatter(std::path::PathBuf::new()));
    }

    // Buscar el segundo ---
    let after_first = &content[3..];
    if let Some(end_pos) = after_first.find(FRONTMATTER_DELIMITER) {
        let before_end = &content[..3 + end_pos];
        let after_end = &content[3 + end_pos..];

        // Agregar campo antes del segundo ---
        let new_line = format!("{}: {}\n", field, value);

        Ok(format!("{}{}{}", before_end, new_line, after_end))
    } else {
        Err(OcError::MissingFrontmatter(std::path::PathBuf::new()))
    }
}

/// Cuenta palabras en el body (excluyendo código y YAML).
pub fn count_words(body: &str) -> usize {
    let mut in_code_block = false;
    let mut word_count = 0;

    for line in body.lines() {
        let trimmed = line.trim();

        // Toggle code block
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        // Skip code blocks
        if in_code_block {
            continue;
        }

        // Skip headings for word count? (optional, keeping them)
        // Skip images and links (contar solo texto)
        let text = trimmed.replace(['#', '*', '_', '`'], " ");

        word_count += text.split_whitespace().count();
    }

    word_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DOC: &str = r#"---
id: "3.1.2"
title: "Documento de Prueba"
parent: "3.1"
breadcrumb: "Módulo 3 > Sección 1 > Documento 2"
status: active
created: "2026-01-15"
last_updated: "2026-01-30"
---

# Contenido del Documento

Este es el body del documento.

## Sección 2

Más contenido aquí.
"#;

    #[test]
    fn test_parse_frontmatter() {
        let result = parse_frontmatter(SAMPLE_DOC).unwrap();
        assert_eq!(result.frontmatter.id, "3.1.2");
        assert_eq!(result.frontmatter.title, "Documento de Prueba");
        assert_eq!(result.frontmatter.parent, Some("3.1".to_string()));
        assert!(result.body.contains("# Contenido del Documento"));
    }

    #[test]
    fn test_missing_frontmatter() {
        let content = "# No hay frontmatter aquí";
        let result = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_body() {
        let body = extract_body(SAMPLE_DOC).unwrap();
        assert!(body.starts_with("# Contenido"));
        assert!(!body.contains("id:"));
    }

    #[test]
    fn test_update_field() {
        let updated = update_field(SAMPLE_DOC, "status", "reviewed").unwrap();
        assert!(updated.contains("status: reviewed"));
        assert!(!updated.contains("status: active"));
    }

    #[test]
    fn test_add_field() {
        let updated = add_field(SAMPLE_DOC, "priority", "high").unwrap();
        assert!(updated.contains("priority: high"));
    }

    #[test]
    fn test_count_words() {
        let body = r#"
# Título

Este es un párrafo con siete palabras aquí.

```rust
// Este código no cuenta
fn main() {}
```

Más texto después del código.
"#;
        let count = count_words(body);
        assert!(count > 10); // Aproximadamente 12-15 palabras
    }

    #[test]
    fn test_document_id_parsing() {
        let result = parse_frontmatter(SAMPLE_DOC).unwrap();
        let doc_id = result.frontmatter.document_id().unwrap();
        assert_eq!(doc_id.module(), 3);
        assert_eq!(doc_id.depth(), 3);
    }
}
