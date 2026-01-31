//! Modelo de documento de documentación.
//!
//! Representa un archivo markdown con su frontmatter, body y metadatos.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::errors::{OcError, OcResult};
use crate::types::{DocumentId, ContentHash, DataPath};
use crate::core::yaml::{YamlFrontmatter, parse_frontmatter, count_words};
use crate::core::links::{Link, extract_links};

// ═══════════════════════════════════════════════════════════════════════════
// DOCUMENT STRUCT
// ═══════════════════════════════════════════════════════════════════════════

/// Un documento de documentación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Ruta al archivo.
    pub path: PathBuf,
    /// Frontmatter YAML parseado.
    pub frontmatter: YamlFrontmatter,
    /// Body del documento (sin frontmatter).
    pub body: String,
    /// Conteo de palabras del body.
    pub word_count: usize,
    /// Hash del contenido.
    pub content_hash: ContentHash,
    /// Enlaces encontrados en el documento.
    #[serde(default)]
    pub links: Vec<Link>,
}

impl Document {
    /// Crea un documento desde un archivo.
    pub fn from_file(path: impl AsRef<Path>) -> OcResult<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| OcError::FileRead {
                path: path.to_path_buf(),
                source: e,
            })?;
        
        Self::from_content(path.to_path_buf(), &content)
    }
    
    /// Crea un documento desde contenido.
    pub fn from_content(path: PathBuf, content: &str) -> OcResult<Self> {
        let parsed = parse_frontmatter(content)?;
        let word_count = count_words(&parsed.body);
        let content_hash = ContentHash::compute(&parsed.body);
        let links = extract_links(&parsed.body);
        
        Ok(Self {
            path,
            frontmatter: parsed.frontmatter,
            body: parsed.body,
            word_count,
            content_hash,
            links,
        })
    }
    
    /// ID del documento.
    pub fn id(&self) -> OcResult<DocumentId> {
        self.frontmatter.document_id()
    }
    
    /// Módulo al que pertenece.
    pub fn module(&self) -> OcResult<u32> {
        Ok(self.id()?.module())
    }
    
    /// Título del documento.
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }
    
    /// ¿Es el documento master (ID = "0")?
    pub fn is_master(&self) -> bool {
        self.frontmatter.is_master()
    }
    
    /// DataPath del documento.
    pub fn data_path(&self) -> DataPath {
        DataPath::from(self.path.clone())
    }
    
    /// ¿Está saludable? (tiene campos requeridos válidos)
    pub fn is_healthy(&self) -> bool {
        self.frontmatter.validate().is_ok() && self.word_count >= 50
    }
    
    /// Profundidad jerárquica.
    pub fn depth(&self) -> usize {
        self.id().map(|id| id.depth()).unwrap_or(0)
    }
    
    /// Parent ID.
    pub fn parent_id(&self) -> Option<DocumentId> {
        self.id().ok().and_then(|id| id.parent())
    }
    
    /// Número de enlaces rotos.
    pub fn broken_link_count(&self) -> usize {
        self.links.iter().filter(|l| !l.is_internal()).count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DOCUMENT BUILDER
// ═══════════════════════════════════════════════════════════════════════════

/// Builder para crear documentos programáticamente.
#[derive(Debug, Default)]
pub struct DocumentBuilder {
    path: Option<PathBuf>,
    frontmatter: Option<YamlFrontmatter>,
    body: Option<String>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
    
    pub fn frontmatter(mut self, fm: YamlFrontmatter) -> Self {
        self.frontmatter = Some(fm);
        self
    }
    
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    
    pub fn build(self) -> OcResult<Document> {
        let path = self.path.unwrap_or_default();
        let frontmatter = self.frontmatter.ok_or_else(|| OcError::MissingField {
            field: "frontmatter".to_string(),
            path: path.clone(),
        })?;
        let body = self.body.unwrap_or_default();
        let word_count = count_words(&body);
        let content_hash = ContentHash::compute(&body);
        let links = extract_links(&body);
        
        Ok(Document {
            path,
            frontmatter,
            body,
            word_count,
            content_hash,
            links,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DOCUMENT COLLECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Colección de documentos.
#[derive(Debug, Default)]
pub struct DocumentCollection {
    docs: Vec<Document>,
}

impl DocumentCollection {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn push(&mut self, doc: Document) {
        self.docs.push(doc);
    }
    
    pub fn len(&self) -> usize {
        self.docs.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.docs.iter()
    }
    
    /// Total de palabras.
    pub fn total_words(&self) -> usize {
        self.docs.iter().map(|d| d.word_count).sum()
    }
    
    /// Documentos por módulo.
    pub fn by_module(&self) -> std::collections::HashMap<u32, Vec<&Document>> {
        let mut map = std::collections::HashMap::new();
        for doc in &self.docs {
            if let Ok(module) = doc.module() {
                map.entry(module).or_insert_with(Vec::new).push(doc);
            }
        }
        map
    }
    
    /// Documentos saludables.
    pub fn healthy(&self) -> impl Iterator<Item = &Document> {
        self.docs.iter().filter(|d| d.is_healthy())
    }
    
    /// Documentos no saludables.
    pub fn unhealthy(&self) -> impl Iterator<Item = &Document> {
        self.docs.iter().filter(|d| !d.is_healthy())
    }
}

impl From<Vec<Document>> for DocumentCollection {
    fn from(docs: Vec<Document>) -> Self {
        Self { docs }
    }
}

impl IntoIterator for DocumentCollection {
    type Item = Document;
    type IntoIter = std::vec::IntoIter<Document>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.docs.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::yaml::YamlFrontmatter;

    fn sample_frontmatter() -> YamlFrontmatter {
        YamlFrontmatter {
            id: "3.1".to_string(),
            title: "Test Document".to_string(),
            status: "active".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_document_builder() {
        let doc = DocumentBuilder::new()
            .path("test.md")
            .frontmatter(sample_frontmatter())
            .body("This is the body content with enough words to pass validation.")
            .build()
            .unwrap();
        
        assert_eq!(doc.title(), "Test Document");
        assert!(doc.word_count > 0);
    }

    #[test]
    fn test_document_id() {
        let doc = DocumentBuilder::new()
            .frontmatter(sample_frontmatter())
            .build()
            .unwrap();
        
        let id = doc.id().unwrap();
        assert_eq!(id.module(), 3);
    }

    #[test]
    fn test_document_collection() {
        let mut coll = DocumentCollection::new();
        coll.push(DocumentBuilder::new()
            .frontmatter(sample_frontmatter())
            .body("Word ".repeat(100))
            .build()
            .unwrap());
        
        assert_eq!(coll.len(), 1);
        assert!(coll.total_words() > 50);
    }

    #[test]
    fn test_is_healthy() {
        let doc = DocumentBuilder::new()
            .frontmatter(sample_frontmatter())
            .body("Word ".repeat(100))
            .build()
            .unwrap();
        
        assert!(doc.is_healthy());
    }
}
