//! Motor de enlaces para detección y validación de links en documentos.
//!
//! Soporta:
//! - Links Obsidian: [[target]] y [[target|alias]]
//! - Links Markdown: [text](url)
//! - Embeds: ![[image]] y ![alt](url)

use std::path::{Path, PathBuf};
use std::collections::HashSet;
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use crate::errors::{OcError, OcResult};

/// Patrón para links Obsidian: [[target]] o [[target|alias]]
static OBSIDIAN_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap()
});

/// Patrón para links Markdown: [text](url)
static MD_LINK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap()
});

/// Patrón para embeds Obsidian: ![[file]]
static OBSIDIAN_EMBED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!\[\[([^\]]+)\]\]").unwrap()
});

/// Patrón para embeds Markdown: ![alt](url)
static MD_EMBED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap()
});

/// Tipo de enlace detectado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    /// Link interno Obsidian [[target]]
    ObsidianInternal,
    /// Link Markdown [text](url)
    Markdown,
    /// Embed Obsidian ![[file]]
    ObsidianEmbed,
    /// Embed Markdown ![alt](url)
    MarkdownEmbed,
    /// Link externo (http/https)
    External,
}

/// Representa un enlace extraído de un documento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Tipo de enlace.
    pub link_type: LinkType,
    /// Target/URL del enlace.
    pub target: String,
    /// Texto visible (alias).
    pub text: Option<String>,
    /// Posición en el contenido (inicio).
    pub position: usize,
    /// Match completo original.
    pub raw: String,
}

impl Link {
    /// ¿Es un enlace interno (no HTTP)?
    pub fn is_internal(&self) -> bool {
        !self.target.starts_with("http://") && 
        !self.target.starts_with("https://") &&
        !self.target.starts_with("mailto:")
    }
    
    /// ¿Es un enlace externo?
    pub fn is_external(&self) -> bool {
        !self.is_internal()
    }
    
    /// ¿Es un embed (imagen/archivo)?
    pub fn is_embed(&self) -> bool {
        matches!(self.link_type, LinkType::ObsidianEmbed | LinkType::MarkdownEmbed)
    }
    
    /// Normaliza el target para comparación.
    pub fn normalized_target(&self) -> String {
        let mut target = self.target.clone();
        
        // Remover anclas (#section)
        if let Some(pos) = target.find('#') {
            target = target[..pos].to_string();
        }
        
        // Agregar .md si no tiene extensión
        if !target.contains('.') && self.is_internal() && !self.is_embed() {
            target.push_str(".md");
        }
        
        target
    }
}

/// Enlace roto detectado.
#[derive(Debug, Clone)]
pub struct BrokenLink {
    /// El enlace que está roto.
    pub link: Link,
    /// Archivo fuente donde se encontró.
    pub source_file: PathBuf,
    /// Razón por la que está roto.
    pub reason: String,
}

/// Extrae todos los enlaces de un contenido.
pub fn extract_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    
    // Obsidian embeds primero (para no confundir con links)
    for cap in OBSIDIAN_EMBED.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        links.push(Link {
            link_type: LinkType::ObsidianEmbed,
            target: cap[1].to_string(),
            text: None,
            position: full_match.start(),
            raw: full_match.as_str().to_string(),
        });
    }
    
    // Markdown embeds
    for cap in MD_EMBED.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        links.push(Link {
            link_type: LinkType::MarkdownEmbed,
            target: cap[2].to_string(),
            text: Some(cap[1].to_string()),
            position: full_match.start(),
            raw: full_match.as_str().to_string(),
        });
    }
    
    // Obsidian links [[target]] o [[target|alias]]
    for cap in OBSIDIAN_LINK.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        // Skip si ya fue capturado como embed
        if content[..full_match.start()].ends_with('!') {
            continue;
        }
        links.push(Link {
            link_type: LinkType::ObsidianInternal,
            target: cap[1].to_string(),
            text: cap.get(2).map(|m| m.as_str().to_string()),
            position: full_match.start(),
            raw: full_match.as_str().to_string(),
        });
    }
    
    // Markdown links [text](url)
    for cap in MD_LINK.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        // Skip si ya fue capturado como embed
        if content[..full_match.start()].ends_with('!') {
            continue;
        }
        let target = cap[2].to_string();
        let link_type = if target.starts_with("http://") || target.starts_with("https://") {
            LinkType::External
        } else {
            LinkType::Markdown
        };
        links.push(Link {
            link_type,
            target,
            text: Some(cap[1].to_string()),
            position: full_match.start(),
            raw: full_match.as_str().to_string(),
        });
    }
    
    // Ordenar por posición
    links.sort_by_key(|l| l.position);
    
    links
}

/// Valida enlaces contra un conjunto de archivos conocidos.
pub fn validate_links(
    links: &[Link],
    source_file: &Path,
    known_files: &HashSet<PathBuf>,
) -> Vec<BrokenLink> {
    let mut broken = Vec::new();
    let source_dir = source_file.parent().unwrap_or(Path::new("."));
    
    for link in links {
        // Skip externos
        if link.is_external() {
            continue;
        }
        
        let target = link.normalized_target();
        
        // Resolver path relativo
        let resolved = source_dir.join(&target);
        let resolved_canonical = resolved.canonicalize().ok();
        
        // Verificar si existe
        let exists = if let Some(canonical) = &resolved_canonical {
            known_files.contains(canonical)
        } else {
            // Buscar por nombre de archivo
            known_files.iter().any(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n == target || n == format!("{}.md", target.trim_end_matches(".md")))
                    .unwrap_or(false)
            })
        };
        
        if !exists {
            broken.push(BrokenLink {
                link: link.clone(),
                source_file: source_file.to_path_buf(),
                reason: format!("Target '{}' not found", target),
            });
        }
    }
    
    broken
}

/// Reemplaza un enlace por otro en el contenido.
pub fn replace_link(content: &str, old_target: &str, new_target: &str) -> String {
    // Reemplazar en Obsidian links
    let result = OBSIDIAN_LINK.replace_all(content, |caps: &regex::Captures| {
        if &caps[1] == old_target {
            if let Some(alias) = caps.get(2) {
                format!("[[{}|{}]]", new_target, alias.as_str())
            } else {
                format!("[[{}]]", new_target)
            }
        } else {
            caps[0].to_string()
        }
    });
    
    // Reemplazar en MD links
    let result = MD_LINK.replace_all(&result, |caps: &regex::Captures| {
        if &caps[2] == old_target {
            format!("[{}]({})", &caps[1], new_target)
        } else {
            caps[0].to_string()
        }
    });
    
    result.to_string()
}

/// Convierte enlaces Obsidian a formato Markdown.
pub fn convert_obsidian_to_md(content: &str) -> String {
    // Convertir embeds ![[file]] → ![file](file)
    let result = OBSIDIAN_EMBED.replace_all(content, |caps: &regex::Captures| {
        let target = &caps[1];
        format!("![{}]({})", target, target)
    });
    
    // Convertir links [[target|alias]] → [alias](target.md)
    let result = OBSIDIAN_LINK.replace_all(&result, |caps: &regex::Captures| {
        let target = &caps[1];
        let text = caps.get(2).map(|m| m.as_str()).unwrap_or(target);
        let target_with_ext = if target.contains('.') {
            target.to_string()
        } else {
            format!("{}.md", target)
        };
        format!("[{}]({})", text, target_with_ext)
    });
    
    result.to_string()
}

/// Extrae solo los targets únicos de los enlaces.
pub fn extract_unique_targets(content: &str) -> HashSet<String> {
    extract_links(content)
        .into_iter()
        .filter(|l| l.is_internal())
        .map(|l| l.normalized_target())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_obsidian_links() {
        let content = "Ver [[documento]] y [[otro|alias]] para más info.";
        let links = extract_links(content);
        
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].target, "documento");
        assert_eq!(links[1].target, "otro");
        assert_eq!(links[1].text, Some("alias".to_string()));
    }

    #[test]
    fn test_extract_md_links() {
        let content = "Ver [texto](archivo.md) y [externo](https://example.com).";
        let links = extract_links(content);
        
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].link_type, LinkType::Markdown);
        assert_eq!(links[1].link_type, LinkType::External);
    }

    #[test]
    fn test_extract_embeds() {
        let content = "Imagen: ![[foto.png]] y ![alt](imagen.jpg).";
        let links = extract_links(content);
        
        assert_eq!(links.len(), 2);
        assert!(links[0].is_embed());
        assert!(links[1].is_embed());
    }

    #[test]
    fn test_convert_obsidian_to_md() {
        let obsidian = "Ver [[documento]] y [[otro|con alias]].";
        let md = convert_obsidian_to_md(obsidian);
        
        assert!(md.contains("[documento](documento.md)"));
        assert!(md.contains("[con alias](otro.md)"));
    }

    #[test]
    fn test_replace_link() {
        let content = "Ver [[viejo]] y [texto](viejo.md).";
        let updated = replace_link(content, "viejo", "nuevo");
        
        assert!(updated.contains("[[nuevo]]"));
        // MD links no se reemplazan (target diferente)
    }

    #[test]
    fn test_validate_links() {
        let links = vec![
            Link {
                link_type: LinkType::ObsidianInternal,
                target: "existe.md".to_string(),
                text: None,
                position: 0,
                raw: "[[existe.md]]".to_string(),
            },
            Link {
                link_type: LinkType::ObsidianInternal,
                target: "noexiste.md".to_string(),
                text: None,
                position: 20,
                raw: "[[noexiste.md]]".to_string(),
            },
        ];
        
        let mut known = HashSet::new();
        known.insert(PathBuf::from("/docs/existe.md"));
        
        let broken = validate_links(&links, Path::new("/docs/source.md"), &known);
        assert_eq!(broken.len(), 1);
        assert!(broken[0].link.target.contains("noexiste"));
    }
}
