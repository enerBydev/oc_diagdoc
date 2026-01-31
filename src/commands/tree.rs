//! Comando tree - Visualización de árbol.
//!
//! Muestra la estructura jerárquica de documentos.

use std::path::PathBuf;
use clap::Parser;
use crate::errors::OcResult;

// ═══════════════════════════════════════════════════════════════════════════
// TREE NODE
// ═══════════════════════════════════════════════════════════════════════════

/// Nodo del árbol para visualización.
#[derive(Debug, Clone)]
pub struct TreeDisplayNode {
    pub id: String,
    pub title: String,
    pub depth: usize,
    pub is_last: bool,
    pub ancestors_are_last: Vec<bool>,
    pub has_children: bool,
    pub status_emoji: String,
}

impl TreeDisplayNode {
    pub fn new(id: impl Into<String>, title: impl Into<String>, depth: usize) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            depth,
            is_last: false,
            ancestors_are_last: Vec::new(),
            has_children: false,
            status_emoji: "📄".to_string(),
        }
    }
    
    /// Genera el prefijo ASCII del árbol.
    pub fn prefix(&self) -> String {
        if self.depth == 0 {
            return String::new();
        }
        
        let mut prefix = String::new();
        
        // Líneas verticales de ancestros
        for &is_last in &self.ancestors_are_last[..self.depth.saturating_sub(1)] {
            prefix.push_str(if is_last { "    " } else { "│   " });
        }
        
        // Conexión al nodo actual
        prefix.push_str(if self.is_last { "└── " } else { "├── " });
        
        prefix
    }
    
    /// Renderiza el nodo completo.
    pub fn render(&self) -> String {
        format!("{}{} {} ({})", 
            self.prefix(), 
            self.status_emoji,
            self.title,
            self.id
        )
    }
}

/// Resultado del árbol.
#[derive(Debug, Clone)]
pub struct TreeResult {
    pub nodes: Vec<TreeDisplayNode>,
    pub total_nodes: usize,
    pub max_depth: usize,
}

impl TreeResult {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            total_nodes: 0,
            max_depth: 0,
        }
    }
    
    /// Renderiza el árbol completo.
    pub fn render(&self) -> String {
        let mut output = String::new();
        
        for node in &self.nodes {
            output.push_str(&node.render());
            output.push('\n');
        }
        
        output.push_str(&format!("\n📊 {} documentos, profundidad máxima: {}\n", 
            self.total_nodes, 
            self.max_depth
        ));
        
        output
    }
}

impl Default for TreeResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TREE COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de visualización de árbol.
#[derive(Parser, Debug, Clone)]
#[command(name = "tree", about = "Visualización de árbol de documentos")]
pub struct TreeCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    
    /// ID del módulo a mostrar.
    #[arg(short, long)]
    pub module: Option<String>,
    
    /// Profundidad máxima.
    #[arg(short, long)]
    pub depth: Option<usize>,
    
    /// Mostrar solo documentos con errores.
    #[arg(long)]
    pub errors_only: bool,
    
    /// Incluir conteo de palabras.
    #[arg(long)]
    pub words: bool,
}

impl TreeCommand {
    /// Ejecuta el comando.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<TreeResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        use regex::Regex;
        use std::collections::HashMap;
        
        let mut result = TreeResult::new();
        
        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;
        
        // Regex para extraer parent_id y title del frontmatter
        let parent_regex = Regex::new(r#"parent_id:\s*["']?([^"'\s\n]+)["']?"#).unwrap();
        let title_regex = Regex::new(r#"title:\s*["']?([^"'\n]+)["']?"#).unwrap();
        
        // Estructura: id -> (title, parent_id, word_count)
        let mut docs: HashMap<String, (String, Option<String>, usize)> = HashMap::new();
        // Estructura: parent_id -> [children_ids]
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Fase 1: Parsear todos los documentos
        for file_path in &files {
            let file_id = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            // Filtrar por módulo si se especificó
            if let Some(ref module_filter) = self.module {
                if !file_id.starts_with(module_filter) {
                    continue;
                }
            }
            
            if let Ok(content) = read_file_content(file_path) {
                let word_count = content.split_whitespace().count();
                
                // Extraer parent_id
                let parent_id = parent_regex.captures(&content)
                    .map(|cap| cap[1].to_string());
                
                // Extraer título o usar el ID
                let title = title_regex.captures(&content)
                    .map(|cap| cap[1].trim().to_string())
                    .unwrap_or_else(|| file_id.clone());
                
                docs.insert(file_id.clone(), (title, parent_id.clone(), word_count));
                
                // Registrar en children_map
                if let Some(ref pid) = parent_id {
                    children_map.entry(pid.clone())
                        .or_default()
                        .push(file_id.clone());
                }
            }
        }
        
        // Fase 2: Encontrar nodos raíz (sin parent_id o parent no existe)
        let mut root_ids: Vec<String> = docs.iter()
            .filter(|(id, (_, parent, _))| {
                parent.is_none() || !docs.contains_key(parent.as_ref().unwrap())
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        root_ids.sort();
        
        // Fase 3: Construir árbol recursivamente
        for (idx, root_id) in root_ids.iter().enumerate() {
            let is_last = idx == root_ids.len() - 1;
            self.build_tree_recursive(
                root_id,
                &docs,
                &children_map,
                0,
                is_last,
                &mut vec![],
                &mut result,
            );
        }
        
        result.total_nodes = result.nodes.len();
        result.max_depth = result.nodes.iter().map(|n| n.depth).max().unwrap_or(0);
        
        Ok(result)
    }
    
    /// Construye el árbol recursivamente.
    fn build_tree_recursive(
        &self,
        id: &str,
        docs: &std::collections::HashMap<String, (String, Option<String>, usize)>,
        children_map: &std::collections::HashMap<String, Vec<String>>,
        depth: usize,
        is_last: bool,
        ancestors_are_last: &mut Vec<bool>,
        result: &mut TreeResult,
    ) {
        // Verificar límite de profundidad
        if let Some(max_depth) = self.depth {
            if depth > max_depth {
                return;
            }
        }
        
        let (title, _, word_count) = docs.get(id).cloned()
            .unwrap_or_else(|| (id.to_string(), None, 0));
        
        let has_children = children_map.contains_key(id);
        
        // Crear nodo
        let mut node = TreeDisplayNode::new(id, &title, depth);
        node.is_last = is_last;
        node.ancestors_are_last = ancestors_are_last.clone();
        node.has_children = has_children;
        
        // Emoji basado en tipo/estado
        node.status_emoji = if depth == 0 {
            "📁".to_string()
        } else if has_children {
            "📂".to_string()
        } else {
            "📄".to_string()
        };
        
        // Agregar conteo de palabras si se pidió
        if self.words {
            node.title = format!("{} ({} words)", node.title, word_count);
        }
        
        result.nodes.push(node);
        
        // Procesar hijos
        if let Some(children) = children_map.get(id) {
            let mut sorted_children: Vec<_> = children.clone();
            sorted_children.sort();
            
            ancestors_are_last.push(is_last);
            
            for (idx, child_id) in sorted_children.iter().enumerate() {
                let child_is_last = idx == sorted_children.len() - 1;
                self.build_tree_recursive(
                    child_id,
                    docs,
                    children_map,
                    depth + 1,
                    child_is_last,
                    ancestors_are_last,
                    result,
                );
            }
            
            ancestors_are_last.pop();
        }
    }
    
    /// Construye un árbol de ejemplo.
    pub fn build_sample_tree() -> TreeResult {
        let mut result = TreeResult::new();
        
        // Ejemplo de árbol
        let nodes = vec![
            ("0", "Contextualizador", 0, false, vec![]),
            ("1", "Módulo Plataforma", 1, false, vec![false]),
            ("1.1", "Visión", 2, false, vec![false, false]),
            ("1.2", "Misión", 2, true, vec![false, true]),
            ("2", "Módulo Usuarios", 1, true, vec![true]),
        ];
        
        for (id, title, depth, is_last, ancestors) in nodes {
            let mut node = TreeDisplayNode::new(id, title, depth);
            node.is_last = is_last;
            node.ancestors_are_last = ancestors;
            result.nodes.push(node);
        }
        
        result.total_nodes = 5;
        result.max_depth = 2;
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_prefix() {
        let mut node = TreeDisplayNode::new("1.1", "Test", 2);
        node.is_last = false;
        node.ancestors_are_last = vec![false, false];
        
        let prefix = node.prefix();
        assert!(prefix.contains("├──"));
    }

    #[test]
    fn test_tree_node_render() {
        let node = TreeDisplayNode::new("1.1", "Test Doc", 0);
        let rendered = node.render();
        
        assert!(rendered.contains("Test Doc"));
        assert!(rendered.contains("1.1"));
    }

    #[test]
    fn test_tree_result_render() {
        let result = TreeCommand::build_sample_tree();
        let output = result.render();
        
        assert!(output.contains("Contextualizador"));
        assert!(output.contains("5 documentos"));
    }

    #[test]
    fn test_sample_tree() {
        let tree = TreeCommand::build_sample_tree();
        assert_eq!(tree.total_nodes, 5);
        assert_eq!(tree.max_depth, 2);
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: TreeCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(&cli.data_dir);
    let result = cmd.run(data_dir)?;
    
    if result.nodes.is_empty() {
        // Mostrar árbol de ejemplo si no hay proyecto
        let sample = TreeCommand::build_sample_tree();
        println!("{}", sample.render());
    } else {
        println!("{}", result.render());
    }
    
    Ok(())
}
