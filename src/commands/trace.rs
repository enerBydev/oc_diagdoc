//! Comando trace - Trazabilidad de documentos.
//!
//! Rastrea referencias y dependencias de un documento.

use std::path::PathBuf;
use std::collections::HashSet;
use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// TRACE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Una referencia rastreada.
#[derive(Debug, Clone)]
pub struct TraceReference {
    pub source: String,
    pub target: String,
    pub ref_type: TraceType,
    pub depth: usize,
}

/// Tipo de referencia.
#[derive(Debug, Clone, PartialEq)]
pub enum TraceType {
    Link,
    Parent,
    Child,
    Embed,
    Backlink,
}

/// Resultado del trace.
#[derive(Debug, Clone)]
pub struct TraceResult {
    pub document_id: String,
    pub references: Vec<TraceReference>,
    pub depth_reached: usize,
}

impl TraceResult {
    pub fn new(doc_id: &str) -> Self {
        Self {
            document_id: doc_id.to_string(),
            references: Vec::new(),
            depth_reached: 0,
        }
    }
    
    pub fn add_reference(&mut self, reference: TraceReference) {
        if reference.depth > self.depth_reached {
            self.depth_reached = reference.depth;
        }
        self.references.push(reference);
    }
    
    pub fn unique_documents(&self) -> HashSet<&str> {
        let mut docs = HashSet::new();
        docs.insert(self.document_id.as_str());
        for r in &self.references {
            docs.insert(&r.source);
            docs.insert(&r.target);
        }
        docs
    }
    
    pub fn by_type(&self, ref_type: TraceType) -> Vec<&TraceReference> {
        self.references.iter().filter(|r| r.ref_type == ref_type).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRACE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de trace.
#[derive(Parser, Debug, Clone)]
#[command(name = "trace", about = "Trazabilidad de documentos")]
pub struct TraceCommand {
    /// ID del documento a rastrear.
    pub document_id: String,
    
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// Profundidad máxima.
    #[arg(short, long, default_value = "3")]
    pub depth: usize,
    
    /// Incluir backlinks.
    #[arg(long)]
    pub backlinks: bool,
}

impl TraceCommand {
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<TraceResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashMap;
        
        let mut result = TraceResult::new(&self.document_id);
        
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        
        // Regex para parent_id y wiki links
        let parent_regex = Regex::new(r#"parent_id:\s*["']?([^"'\s\n]+)["']?"#).unwrap();
        let wiki_link = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        
        // Construir mapas de relaciones
        let mut parent_map: HashMap<String, String> = HashMap::new(); // id -> parent_id
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new(); // id -> [children]
        let mut links_from: HashMap<String, Vec<String>> = HashMap::new(); // id -> [links salientes]
        let mut links_to: HashMap<String, Vec<String>> = HashMap::new(); // id -> [backlinks]
        
        // Fase 1: Parsear relaciones
        for file_path in &files {
            let file_id = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            if let Ok(content) = read_file_content(file_path) {
                // Extraer parent_id
                if let Some(cap) = parent_regex.captures(&content) {
                    let parent_id = cap[1].to_string();
                    parent_map.insert(file_id.clone(), parent_id.clone());
                    children_map.entry(parent_id).or_default().push(file_id.clone());
                }
                
                // Extraer wiki links
                for cap in wiki_link.captures_iter(&content) {
                    let target = cap[1].split('/').last().unwrap_or(&cap[1])
                        .split('|').next().unwrap_or(&cap[1]).trim().to_string();
                    
                    if target != file_id {
                        links_from.entry(file_id.clone()).or_default().push(target.clone());
                        links_to.entry(target).or_default().push(file_id.clone());
                    }
                }
            }
        }
        
        // Fase 2: Trazar ancestros (subiendo por parent_id)
        self.trace_ancestors(&self.document_id, &parent_map, 1, &mut result);
        
        // Fase 3: Trazar descendientes (bajando por children)
        self.trace_descendants(&self.document_id, &children_map, 1, &mut result);
        
        // Fase 4: Trazar links salientes
        if let Some(links) = links_from.get(&self.document_id) {
            for target in links {
                if result.references.len() < 100 { // Limite
                    result.add_reference(TraceReference {
                        source: self.document_id.clone(),
                        target: target.clone(),
                        ref_type: TraceType::Link,
                        depth: 1,
                    });
                }
            }
        }
        
        // Fase 5: Trazar backlinks (quien me referencia)
        if self.backlinks {
            if let Some(backlinks) = links_to.get(&self.document_id) {
                for source in backlinks {
                    if result.references.len() < 100 { // Limite
                        result.add_reference(TraceReference {
                            source: source.clone(),
                            target: self.document_id.clone(),
                            ref_type: TraceType::Backlink,
                            depth: 1,
                        });
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Traza ancestros recursivamente.
    fn trace_ancestors(
        &self,
        doc_id: &str,
        parent_map: &std::collections::HashMap<String, String>,
        depth: usize,
        result: &mut TraceResult,
    ) {
        if depth > self.depth {
            return;
        }
        
        if let Some(parent_id) = parent_map.get(doc_id) {
            result.add_reference(TraceReference {
                source: doc_id.to_string(),
                target: parent_id.clone(),
                ref_type: TraceType::Parent,
                depth,
            });
            self.trace_ancestors(parent_id, parent_map, depth + 1, result);
        }
    }
    
    /// Traza descendientes recursivamente.
    fn trace_descendants(
        &self,
        doc_id: &str,
        children_map: &std::collections::HashMap<String, Vec<String>>,
        depth: usize,
        result: &mut TraceResult,
    ) {
        if depth > self.depth {
            return;
        }
        
        if let Some(children) = children_map.get(doc_id) {
            for child_id in children {
                result.add_reference(TraceReference {
                    source: doc_id.to_string(),
                    target: child_id.clone(),
                    ref_type: TraceType::Child,
                    depth,
                });
                self.trace_descendants(child_id, children_map, depth + 1, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_result_new() {
        let result = TraceResult::new("1.1");
        assert_eq!(result.document_id, "1.1");
        assert_eq!(result.depth_reached, 0);
    }

    #[test]
    fn test_add_reference() {
        let mut result = TraceResult::new("1.1");
        result.add_reference(TraceReference {
            source: "1.1".to_string(),
            target: "1.2".to_string(),
            ref_type: TraceType::Link,
            depth: 1,
        });
        
        assert_eq!(result.references.len(), 1);
        assert_eq!(result.depth_reached, 1);
    }

    #[test]
    fn test_unique_documents() {
        let mut result = TraceResult::new("1.1");
        result.add_reference(TraceReference {
            source: "1.1".to_string(),
            target: "1.2".to_string(),
            ref_type: TraceType::Link,
            depth: 1,
        });
        
        assert_eq!(result.unique_documents().len(), 2);
    }

    #[test]
    fn test_by_type() {
        let mut result = TraceResult::new("1.1");
        result.add_reference(TraceReference {
            source: "1.1".to_string(),
            target: "1.2".to_string(),
            ref_type: TraceType::Link,
            depth: 1,
        });
        result.add_reference(TraceReference {
            source: "1.1".to_string(),
            target: "0".to_string(),
            ref_type: TraceType::Parent,
            depth: 1,
        });
        
        assert_eq!(result.by_type(TraceType::Link).len(), 1);
        assert_eq!(result.by_type(TraceType::Parent).len(), 1);
    }
}

/// Función run para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: TraceCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;
    
    println!("🔍 Trace de: {}", result.document_id);
    println!("📊 {} referencias encontradas", result.references.len());
    println!("📈 Profundidad: {}", result.depth_reached);
    println!("📄 {} documentos únicos", result.unique_documents().len());
    
    for r in &result.references {
        let icon = match r.ref_type {
            TraceType::Link => "🔗",
            TraceType::Parent => "⬆️",
            TraceType::Child => "⬇️",
            TraceType::Embed => "📎",
            TraceType::Backlink => "↩️",
        };
        println!("  {} {} → {}", icon, r.source, r.target);
    }
    
    Ok(())
}
