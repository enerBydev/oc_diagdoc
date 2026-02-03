//! Comando tree - Visualización de árbol.
//!
//! Muestra la estructura jerárquica de documentos.

use crate::errors::OcResult;
use clap::Parser;
use std::path::PathBuf;

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
    // L2: Campos avanzados
    pub word_count: usize,
    pub link_count: usize,
    pub is_orphan: bool,
    pub doc_type: String, // master, module_root, branch, leaf
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
            word_count: 0,
            link_count: 0,
            is_orphan: false,
            doc_type: "leaf".to_string(),
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
        prefix.push_str(if self.is_last {
            "└── "
        } else {
            "├── "
        });

        prefix
    }

    /// Renderiza el nodo completo.
    pub fn render(&self) -> String {
        format!(
            "{}{} {} ({})",
            self.prefix(),
            self.status_emoji,
            self.title,
            self.id
        )
    }

    /// L2: Renderiza con colores ANSI.
    pub fn render_colored(&self) -> String {
        let color_code = match self.doc_type.as_str() {
            "master" => "\x1b[1;35m",      // Magenta bold
            "module_root" => "\x1b[1;36m", // Cyan bold
            "branch" => "\x1b[1;33m",      // Yellow bold
            _ => "\x1b[0m",                // Reset
        };
        let reset = "\x1b[0m";
        let orphan_mark = if self.is_orphan {
            " ⚠️ HUÉRFANO"
        } else {
            ""
        };

        format!(
            "{}{}{}{} {} ({}){}",
            self.prefix(),
            color_code,
            self.status_emoji,
            reset,
            self.title,
            self.id,
            orphan_mark
        )
    }

    /// L2: Renderiza con stats (palabras, links).
    pub fn render_with_stats(&self) -> String {
        format!(
            "{}{} {} ({}) [{} words, {} links]{}",
            self.prefix(),
            self.status_emoji,
            self.title,
            self.id,
            self.word_count,
            self.link_count,
            if self.is_orphan { " ⚠️" } else { "" }
        )
    }
}

/// Resultado del árbol.
#[derive(Debug, Clone)]
pub struct TreeResult {
    pub nodes: Vec<TreeDisplayNode>,
    pub total_nodes: usize,
    pub max_depth: usize,
    // L2: Estadísticas adicionales
    pub orphans_count: usize,
    pub total_words: usize,
}

impl TreeResult {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            total_nodes: 0,
            max_depth: 0,
            orphans_count: 0,
            total_words: 0,
        }
    }

    /// Renderiza el árbol completo.
    pub fn render(&self) -> String {
        let mut output = String::new();

        for node in &self.nodes {
            output.push_str(&node.render());
            output.push('\n');
        }

        output.push_str(&format!(
            "\n📊 {} documentos, profundidad máxima: {}\n",
            self.total_nodes, self.max_depth
        ));

        output
    }

    /// L2: Renderiza con colores ANSI.
    pub fn render_colored(&self) -> String {
        let mut output = String::new();

        for node in &self.nodes {
            output.push_str(&node.render_colored());
            output.push('\n');
        }

        output.push_str(&format!(
            "\n📊 {} docs | 🌳 depth {} | ⚠️ {} huérfanos\n",
            self.total_nodes, self.max_depth, self.orphans_count
        ));

        output
    }

    /// L2: Renderiza con estadísticas por nodo.
    pub fn render_with_stats(&self) -> String {
        let mut output = String::new();

        for node in &self.nodes {
            output.push_str(&node.render_with_stats());
            output.push('\n');
        }

        output.push_str(&format!(
            "\n📊 {} docs | {} words | depth {} | ⚠️ {} huérfanos\n",
            self.total_nodes, self.total_words, self.max_depth, self.orphans_count
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

    // L2: Flags avanzados
    /// Mostrar con colores ANSI por tipo.
    #[arg(long)]
    pub color: bool,

    /// Mostrar estadísticas por nodo (palabras, links).
    #[arg(long)]
    pub stats: bool,

    /// Filtrar por tipo de documento (master, module_root, branch, leaf).
    #[arg(long, value_name = "TYPE")]
    pub doc_type: Option<String>,

    /// Mostrar solo nodos huérfanos.
    #[arg(long)]
    pub orphans_only: bool,

    // AN-04 FIX: Root filter
    /// ID del nodo raíz desde donde mostrar (alternativa a --module).
    #[arg(long)]
    pub root: Option<String>,

    // P1: Nuevas flags de paridad con Python v16
    /// Mostrar status (draft/published/review) junto a cada nodo.
    #[arg(long)]
    pub show_status: bool,

    /// Formato de salida: ascii, json, md
    #[arg(long, default_value = "ascii")]
    pub format: String,

    /// Guardar resultado en archivo.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}


impl TreeCommand {
    /// Ejecuta el comando.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<TreeResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        
        use std::collections::HashMap;

        let mut result = TreeResult::new();

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        // Regex para extraer parent_id y title del frontmatter
        use crate::core::patterns::{RE_PARENT_ID, RE_TITLE};
        let parent_regex = &*RE_PARENT_ID;
        let title_regex = &*RE_TITLE;

        // Estructura: id -> (title, parent_id, word_count)
        let mut docs: HashMap<String, (String, Option<String>, usize)> = HashMap::new();
        // Estructura: parent_id -> [children_ids]
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();

        // Fase 1: Parsear todos los documentos
        for file_path in &files {
            let file_id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            // Filtrar por módulo si se especificó
            if let Some(ref module_filter) = self.module {
                if !file_id.starts_with(module_filter) {
                    continue;
                }
            }

            // AN-04 FIX + P1-A1: Filtrar por root con matching flexible
            if let Some(ref root_filter) = self.root {
                // Normalizar: "1.0" debe coincidir con "1.0.", "1.0.1", "1.0 titulo", etc.
                let normalized_filter = if root_filter.ends_with('.') {
                    root_filter.clone()
                } else {
                    format!("{}.", root_filter)
                };
                
                // Matching flexible: prefijo exacto O prefijo normalizado O file_id == filter
                let matches = file_id.starts_with(root_filter) 
                    || file_id.starts_with(&normalized_filter)
                    || file_id == *root_filter;
                
                if !matches {
                    continue;
                }
            }

            if let Ok(content) = read_file_content(file_path) {
                let word_count = content.split_whitespace().count();

                // Extraer parent_id
                let parent_id = parent_regex
                    .captures(&content)
                    .map(|cap| cap[1].to_string());

                // Extraer título o usar el ID
                let title = title_regex
                    .captures(&content)
                    .map(|cap| cap[1].trim().to_string())
                    .unwrap_or_else(|| file_id.clone());

                docs.insert(file_id.clone(), (title, parent_id.clone(), word_count));

                // Registrar en children_map
                if let Some(ref pid) = parent_id {
                    children_map
                        .entry(pid.clone())
                        .or_default()
                        .push(file_id.clone());
                }
            }
        }

        // Fase 2: Encontrar nodos raíz (sin parent_id o parent no existe)
        let mut root_ids: Vec<String> = docs
            .iter()
            .filter(|(_id, (_, parent, _))| {
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
        // L2: Calcular stats adicionales
        result.orphans_count = result.nodes.iter().filter(|n| n.is_orphan).count();
        result.total_words = result.nodes.iter().map(|n| n.word_count).sum();

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

        let (title, parent_id, word_count) = docs
            .get(id)
            .cloned()
            .unwrap_or_else(|| (id.to_string(), None, 0));

        let has_children = children_map.contains_key(id);

        // L2: Determinar tipo de documento
        let doc_type = if depth == 0 && id.starts_with("0.") {
            "master"
        } else if has_children && depth <= 1 {
            "module_root"
        } else if has_children {
            "branch"
        } else {
            "leaf"
        }
        .to_string();

        // L2: Detectar huérfano (parent_id existe pero parent no en docs)
        let is_orphan = parent_id
            .as_ref()
            .map(|pid| !docs.contains_key(pid))
            .unwrap_or(false);

        // Filtrar por tipo si se especificó
        if let Some(ref type_filter) = self.doc_type {
            if doc_type != *type_filter {
                return;
            }
        }

        // Filtrar solo huérfanos si se pidió
        if self.orphans_only && !is_orphan {
            // Continuar solo para procesar hijos (por si hay huérfanos anidados)
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
            return;
        }

        // Crear nodo
        let mut node = TreeDisplayNode::new(id, &title, depth);
        node.is_last = is_last;
        node.ancestors_are_last = ancestors_are_last.clone();
        node.has_children = has_children;
        node.word_count = word_count;
        node.is_orphan = is_orphan;
        node.doc_type = doc_type;

        // Emoji basado en tipo/estado
        node.status_emoji = if is_orphan {
            "⚠️".to_string()
        } else if depth == 0 {
            "📁".to_string()
        } else if has_children {
            "📂".to_string()
        } else {
            "📄".to_string()
        };

        // Agregar conteo de palabras en título si se pidió
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
    // F1.2: Priorizar cmd.path sobre cli.data_dir
    let default_dir = std::path::PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);
    let result = cmd.run(data_dir)?;

    if result.nodes.is_empty() {
        // Mostrar árbol de ejemplo si no hay proyecto
        let sample = TreeCommand::build_sample_tree();
        println!("{}", sample.render());
    } else {
        // L2: Seleccionar método de renderizado según flags
        let output = if cmd.stats {
            result.render_with_stats()
        } else if cmd.color {
            result.render_colored()
        } else {
            result.render()
        };
        println!("{}", output);
    }

    Ok(())
}
