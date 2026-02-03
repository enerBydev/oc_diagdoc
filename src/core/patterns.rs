//! Patrones regex precompilados con Lazy initialization.
//!
//! Este módulo centraliza todos los patrones regex usados en oc_diagdoc,
//! compilándolos una sola vez usando `once_cell::sync::Lazy` para máximo rendimiento.
//!
//! ## Categorías de Patrones
//!
//! | Categoría | Prefijo | Uso |
//! |-----------|---------|-----|
//! | Frontmatter | `RE_` | Parsing de campos YAML |
//! | Links | `RE_WIKI_`, `RE_MD_` | Extracción de enlaces |
//! | Contenido | `RE_EMBED_`, `RE_IMAGE_` | Parsing de contenido |
//! | Tablas | `RE_TABLE_` | Validación de tablas MD |
//!
//! ## Ejemplo
//!
//! ```rust,ignore
//! use oc_diagdoc_lib::core::patterns::RE_DOCUMENT_ID;
//!
//! let content = "document_id: DOC-001";
//! if let Some(caps) = RE_DOCUMENT_ID.captures(content) {
//!     println!("ID: {}", &caps[1]);
//! }
//! ```

use once_cell::sync::Lazy;
use regex::Regex;

// ═══════════════════════════════════════════════════════════════════════════
// PATRONES DE FRONTMATTER
// ═══════════════════════════════════════════════════════════════════════════

/// Captura `document_id: <valor>` del frontmatter.
pub static RE_DOCUMENT_ID: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"document_id:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `parent_id: <valor>` del frontmatter.
pub static RE_PARENT_ID: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"parent_id:\s*["']?([^"'\s\n]+)["']?"#).unwrap());

/// Captura `module: <valor>` del frontmatter.
pub static RE_MODULE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"module:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `title: <valor>` del frontmatter.
pub static RE_TITLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"title:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `status: <valor>` del frontmatter.
pub static RE_STATUS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"status:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `last_updated: <valor>` del frontmatter.
pub static RE_LAST_UPDATED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"last_updated:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `content_hash: <valor>` del frontmatter.
pub static RE_CONTENT_HASH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"content_hash:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Captura `children_count: <número>` del frontmatter.
pub static RE_CHILDREN_COUNT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"children_count:\s*(\d+)"#).unwrap());

/// Captura `total_children: <número>` del frontmatter.
pub static RE_TOTAL_CHILDREN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"total_children:\s*(\d+)"#).unwrap());

/// Captura `type: <valor>` del frontmatter.
pub static RE_TYPE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"type:\s*["']?([^"'\n]+)["']?"#).unwrap());

/// Detecta `draft: true` en frontmatter.
pub static RE_DRAFT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"draft:\s*true"#).unwrap());

// ═══════════════════════════════════════════════════════════════════════════
// PATRONES DE LINKS
// ═══════════════════════════════════════════════════════════════════════════

/// Wiki link simple: `[[target]]` → captura "target"
pub static RE_WIKI_LINK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[([^\]]+)\]\]").unwrap());

/// Wiki link con posible alias: `[[target|alias]]` → captura "target" (ignora alias)
pub static RE_WIKI_LINK_WITH_ALIAS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap());

/// Wiki link completo: `[[target|alias]]` → captura ambos grupos
pub static RE_WIKI_LINK_FULL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap());

/// Markdown link: `[text](url)` → captura text y url
pub static RE_MD_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap());

/// Markdown link a archivo .md: `[text](file.md)` → captura text y file.md
pub static RE_MD_LINK_TO_MD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)").unwrap());

// ═══════════════════════════════════════════════════════════════════════════
// PATRONES DE CONTENIDO EMBEBIDO
// ═══════════════════════════════════════════════════════════════════════════

/// Obsidian embed: `![[target]]` → captura "target"
pub static RE_OBSIDIAN_EMBED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[\[([^\]]+)\]\]").unwrap());

/// Markdown embed/imagen: `![alt](src)` → captura alt y src
pub static RE_MD_EMBED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());

/// Imagen markdown (alias de RE_MD_EMBED para claridad)
pub static RE_IMAGE: Lazy<Regex> = Lazy::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());

/// Imagen sin alt text: `![](src)`
pub static RE_IMAGE_EMPTY_ALT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[\]\([^)]+\)").unwrap());

/// Bloque mermaid: ```mermaid ... ```
pub static RE_MERMAID: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"```mermaid\s*([\s\S]*?)```").unwrap());

// ═══════════════════════════════════════════════════════════════════════════
// PATRONES DE TABLAS Y LINTING
// ═══════════════════════════════════════════════════════════════════════════

/// Fila de tabla markdown: `| ... |`
pub static RE_TABLE_ROW: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^\|.+\|$").unwrap());

/// Separador de tabla: `|---|---|`
pub static RE_TABLE_SEPARATOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\|[-:\s|]+\|$").unwrap());

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_patterns_compile() {
        // Forzar inicialización de todos los patrones
        assert!(RE_DOCUMENT_ID.is_match("document_id: test"));
        assert!(RE_PARENT_ID.is_match("parent_id: PAR-001"));
        assert!(RE_MODULE.is_match("module: Core"));
        assert!(RE_TITLE.is_match("title: Test Title"));
        assert!(RE_STATUS.is_match("status: draft"));
        assert!(RE_LAST_UPDATED.is_match("last_updated: 2024-01-01"));
        assert!(RE_CONTENT_HASH.is_match("content_hash: abc123"));
        assert!(RE_CHILDREN_COUNT.is_match("children_count: 5"));
        assert!(RE_TOTAL_CHILDREN.is_match("total_children: 10"));
        assert!(RE_TYPE.is_match("type: document"));
        assert!(RE_DRAFT.is_match("draft: true"));
    }

    #[test]
    fn test_link_patterns() {
        // Wiki links
        assert!(RE_WIKI_LINK.is_match("[[target]]"));
        assert!(RE_WIKI_LINK.is_match("[[target|alias]]"));

        // Wiki link with alias - captura solo target
        let caps = RE_WIKI_LINK_WITH_ALIAS
            .captures("[[target|alias]]")
            .unwrap();
        assert_eq!(&caps[1], "target");

        // Markdown links
        assert!(RE_MD_LINK.is_match("[text](url)"));
        let caps = RE_MD_LINK.captures("[text](url)").unwrap();
        assert_eq!(&caps[1], "text");
        assert_eq!(&caps[2], "url");
    }

    #[test]
    fn test_embed_patterns() {
        assert!(RE_OBSIDIAN_EMBED.is_match("![[image.png]]"));
        assert!(RE_MD_EMBED.is_match("![alt](src.png)"));
        assert!(RE_IMAGE.is_match("![](image.jpg)"));
        assert!(RE_IMAGE_EMPTY_ALT.is_match("![](image.jpg)"));
        assert!(!RE_IMAGE_EMPTY_ALT.is_match("![alt](image.jpg)"));
    }

    #[test]
    fn test_mermaid_pattern() {
        let content = "```mermaid\ngraph TD\n  A --> B\n```";
        let caps = RE_MERMAID.captures(content).unwrap();
        assert!(caps[1].contains("graph TD"));
    }

    #[test]
    fn test_table_patterns() {
        assert!(RE_TABLE_ROW.is_match("| col1 | col2 |"));
        assert!(RE_TABLE_SEPARATOR.is_match("|---|---|"));
        assert!(RE_TABLE_SEPARATOR.is_match("| :--- | ---: |"));
    }

    #[test]
    fn test_frontmatter_captures() {
        // Document ID with quotes
        let caps = RE_DOCUMENT_ID.captures("document_id: 'DOC-001'").unwrap();
        assert_eq!(&caps[1], "DOC-001");

        // Document ID without quotes
        let caps = RE_DOCUMENT_ID.captures("document_id: DOC-002").unwrap();
        assert_eq!(&caps[1], "DOC-002");
    }

    #[test]
    fn test_wiki_link_full() {
        // Con alias
        let caps = RE_WIKI_LINK_FULL.captures("[[target|display]]").unwrap();
        assert_eq!(&caps[1], "target");
        assert_eq!(&caps[2], "display");

        // Sin alias
        let caps = RE_WIKI_LINK_FULL.captures("[[target]]").unwrap();
        assert_eq!(&caps[1], "target");
        assert!(caps.get(2).is_none());
    }
}
