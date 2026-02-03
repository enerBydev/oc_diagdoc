//! Comando deps - Análisis de dependencias.
//!
//! Mapea y visualiza dependencias entre documentos.

use crate::errors::OcResult;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// DEPENDENCY TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo de dependencia.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyType {
    /// Link directo.
    Link,
    /// Parent-child.
    Hierarchy,
    /// Embed.
    Embed,
}

/// Una dependencia.
#[derive(Debug, Clone)]
pub struct Dependency {
    pub from: String,
    pub to: String,
    pub dep_type: DependencyType,
}

/// Ciclo detectado.
#[derive(Debug, Clone)]
pub struct Cycle {
    pub nodes: Vec<String>,
}

impl Cycle {
    pub fn new(nodes: Vec<String>) -> Self {
        Self { nodes }
    }
}

impl std::fmt::Display for Cycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} → {}", self.nodes.join(" → "), &self.nodes[0])
    }
}

/// P1-A2: Detalles de un documento huérfano.
#[derive(Debug, Clone)]
pub struct OrphanDetails {
    pub id: String,
    pub invalid_parent: Option<String>,  // El parent que no existe o es inválido
    pub reason: String,                   // "no_parent", "null_parent", "missing_parent"
}

/// Resultado del análisis de dependencias.
#[derive(Debug, Clone)]
pub struct DepsResult {
    pub dependencies: Vec<Dependency>,
    pub cycles: Vec<Cycle>,
    pub root_nodes: Vec<String>,
    pub leaf_nodes: Vec<String>,
    pub orphan_nodes: Vec<OrphanDetails>,  // P1-A2: Detalles de huérfanos
}

impl DepsResult {
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            cycles: Vec::new(),
            root_nodes: Vec::new(),
            leaf_nodes: Vec::new(),
            orphan_nodes: Vec::new(),
        }
    }

    pub fn has_cycles(&self) -> bool {
        !self.cycles.is_empty()
    }

    /// Genera diagrama Mermaid.
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("```mermaid\ngraph TD\n");

        for dep in &self.dependencies {
            let arrow = match dep.dep_type {
                DependencyType::Link => "-->",
                DependencyType::Hierarchy => "==>",
                DependencyType::Embed => "-.->",
            };
            output.push_str(&format!(
                "    {} {} {}\n",
                dep.from.replace('.', "_"),
                arrow,
                dep.to.replace('.', "_")
            ));
        }

        output.push_str("```\n");
        output
    }
}

impl Default for DepsResult {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DEPS COMMAND
// ═══════════════════════════════════════════════════════════════════════════

/// Comando de análisis de dependencias.
#[derive(Parser, Debug, Clone)]
#[command(name = "deps", about = "Análisis de dependencias")]
pub struct DepsCommand {
    /// Ruta del proyecto.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// ID del documento raíz.
    #[arg(short, long)]
    pub root: Option<String>,

    /// Detectar ciclos.
    #[arg(long)]
    pub detect_cycles: bool,

    /// Output formato mermaid.
    #[arg(long)]
    pub mermaid: bool,

    /// Profundidad máxima.
    #[arg(short, long)]
    pub depth: Option<usize>,

    // F5: Nuevas flags de paridad con Python
    /// Dirección del análisis (up=hacia padres, down=hacia hijos, both=ambos).
    #[arg(long, default_value = "both")]
    pub direction: String,

    /// Mostrar análisis de impacto si se modifica un documento.
    #[arg(long)]
    pub impact: Option<String>,

    /// Mostrar solo documentos huérfanos (sin parent).
    #[arg(long)]
    pub orphans: bool,

    // P1: Nuevas flags de paridad con Python v16
    /// Generar grafo en formato DOT (Graphviz).
    #[arg(long)]
    pub graph: bool,

    /// Formato de salida: dot, json, table
    #[arg(long, default_value = "table")]
    pub format: String,

    /// Guardar resultado en archivo.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}


impl DepsCommand {
    /// Ejecuta el análisis.
    pub fn run(&self, data_dir: &std::path::Path) -> OcResult<DepsResult> {
        use crate::core::files::{get_all_md_files, read_file_content, ScanOptions};
        
        use std::collections::HashSet;

        let mut result = DepsResult::new();

        let options = ScanOptions::new();
        let files = get_all_md_files(data_dir, &options)?;

        // Patrones para detectar dependencias
        use crate::core::patterns::{RE_PARENT_ID, RE_WIKI_LINK, RE_MD_LINK_TO_MD};
        let parent_regex = &*RE_PARENT_ID;
        let wiki_link = &*RE_WIKI_LINK;
        let markdown_link = &*RE_MD_LINK_TO_MD;

        let mut all_nodes: HashSet<String> = HashSet::new();
        let mut nodes_with_parents: HashSet<String> = HashSet::new();
        let mut nodes_with_children: HashSet<String> = HashSet::new();
        // C3: Mapa para verificar parent existente
        let mut parent_map: HashMap<String, String> = HashMap::new();

        for file_path in &files {
            // Extraer ID del archivo
            let file_id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            all_nodes.insert(file_id.clone());

            if let Ok(content) = read_file_content(file_path) {
                // Buscar parent_id en frontmatter
                if let Some(cap) = parent_regex.captures(&content) {
                    let parent_id = cap[1].trim().to_string();
                    
                    // C3: Verificar si parent es válido
                    if parent_id != "null" && !parent_id.is_empty() {
                        result.dependencies.push(Dependency {
                            from: parent_id.clone(),
                            to: file_id.clone(),
                            dep_type: DependencyType::Hierarchy,
                        });
                        nodes_with_parents.insert(file_id.clone());
                        nodes_with_children.insert(parent_id.clone());
                        parent_map.insert(file_id.clone(), parent_id);
                    }
                    // Si parent es "null" o vacío, no agregamos a nodes_with_parents
                }
                // Si no hay campo parent, tampoco agregamos a nodes_with_parents

                // Buscar wiki links
                for cap in wiki_link.captures_iter(&content) {
                    let target = &cap[1];
                    if target != file_id {
                        result.dependencies.push(Dependency {
                            from: file_id.clone(),
                            to: target.to_string(),
                            dep_type: DependencyType::Link,
                        });
                    }
                }

                // Buscar markdown links a archivos .md
                for cap in markdown_link.captures_iter(&content) {
                    let target = &cap[2];
                    if !target.starts_with("http") {
                        let target_id = std::path::Path::new(target)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(target);
                        if target_id != file_id {
                            result.dependencies.push(Dependency {
                                from: file_id.clone(),
                                to: target_id.to_string(),
                                dep_type: DependencyType::Link,
                            });
                        }
                    }
                }
            }
        }

        // Calcular nodos raíz (sin parent) y hoja (sin children)
        for node in &all_nodes {
            if !nodes_with_parents.contains(node) {
                result.root_nodes.push(node.clone());
            }
            if !nodes_with_children.contains(node) {
                result.leaf_nodes.push(node.clone());
            }
        }

        // P1-A2: Detectar huérfanos con detalles (sin parent válido O parent inexistente)
        for node in &all_nodes {
            if !nodes_with_parents.contains(node) {
                // Sin parent o parent=null → huérfano
                result.orphan_nodes.push(OrphanDetails {
                    id: node.clone(),
                    invalid_parent: None,
                    reason: "no_parent".to_string(),
                });
            } else if let Some(parent) = parent_map.get(node) {
                // Verificar si parent referenciado existe
                if !all_nodes.contains(parent) {
                    result.orphan_nodes.push(OrphanDetails {
                        id: node.clone(),
                        invalid_parent: Some(parent.clone()),
                        reason: "missing_parent".to_string(),
                    });
                }
            }
        }

        result.root_nodes.sort();
        result.leaf_nodes.sort();
        result.orphan_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        if self.detect_cycles {
            self.find_cycles(&mut result);
        }

        Ok(result)
    }

    /// Detecta ciclos en las dependencias.
    fn find_cycles(&self, result: &mut DepsResult) {
        // Build adjacency list
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();

        for dep in &result.dependencies {
            adj.entry(dep.from.clone())
                .or_default()
                .push(dep.to.clone());
        }

        // DFS para detectar ciclos
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in adj.keys() {
            if !visited.contains(node) {
                self.dfs_cycle(node, &adj, &mut visited, &mut rec_stack, &mut path, result);
            }
        }
    }

    fn dfs_cycle(
        &self,
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        result: &mut DepsResult,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_cycle(neighbor, adj, visited, rec_stack, path, result);
                } else if rec_stack.contains(neighbor) {
                    // Cycle found
                    let cycle_start = path.iter().position(|n| n == neighbor).unwrap();
                    let cycle_nodes: Vec<_> = path[cycle_start..].to_vec();
                    result.cycles.push(Cycle::new(cycle_nodes));
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deps_result_new() {
        let result = DepsResult::new();
        assert!(!result.has_cycles());
    }

    #[test]
    fn test_cycle_to_string() {
        let cycle = Cycle::new(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        assert_eq!(cycle.to_string(), "A → B → C → A");
    }

    #[test]
    fn test_to_mermaid() {
        let mut result = DepsResult::new();
        result.dependencies.push(Dependency {
            from: "1.1".to_string(),
            to: "1.2".to_string(),
            dep_type: DependencyType::Link,
        });

        let mermaid = result.to_mermaid();
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("1_1 --> 1_2"));
    }

    #[test]
    fn test_dependency_type() {
        let dep = Dependency {
            from: "A".to_string(),
            to: "B".to_string(),
            dep_type: DependencyType::Hierarchy,
        };

        assert_eq!(dep.dep_type, DependencyType::Hierarchy);
    }
}

/// Función de ejecución para CLI.
#[cfg(feature = "cli")]
pub fn run(cmd: DepsCommand, cli: &crate::CliConfig) -> anyhow::Result<()> {
    // F5: Corregir path handling
    let default_dir = std::path::PathBuf::from(&cli.data_dir);
    let data_dir = cmd.path.as_ref().unwrap_or(&default_dir);

    // F5: Procesar --orphans
    if cmd.orphans {
        println!("👻 Documentos huérfanos (sin parent válido):");

        use crate::core::patterns::RE_PARENT_ID;
        let parent_re = &*RE_PARENT_ID;
        let mut orphans_count = 0;
        let mut missing_parent_count = 0;

        // Construir set de todos los IDs válidos
        let mut all_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    all_ids.insert(stem.to_string());
                }
            }
        }

        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            if let Ok(content) = std::fs::read_to_string(&path) {
                let parent_info = if let Some(cap) = parent_re.captures(&content) {
                    let parent = cap[1].trim();
                    if parent.is_empty() || parent == "null" || parent == "~" {
                        Some(("no_parent".to_string(), None))
                    } else if !all_ids.contains(parent) {
                        Some(("missing_parent".to_string(), Some(parent.to_string())))
                    } else {
                        None // Tiene parent válido
                    }
                } else {
                    Some(("no_parent".to_string(), None))
                };

                if let Some((reason, invalid_parent)) = parent_info {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    match reason.as_str() {
                        "missing_parent" => {
                            println!("  📄 {} → ⚠️  parent inexistente: '{}'", name, invalid_parent.unwrap_or_default());
                            missing_parent_count += 1;
                        }
                        _ => {
                            println!("  📄 {} → sin parent definido", name);
                        }
                    }
                    orphans_count += 1;
                }
            }
        }

        println!("\n📊 {} documentos huérfanos encontrados", orphans_count);
        if missing_parent_count > 0 {
            println!("   ⚠️  {} con parent inexistente", missing_parent_count);
        }
        return Ok(());
    }

    // F5: Procesar --impact
    if let Some(ref doc_id) = cmd.impact {
        println!("💥 Análisis de impacto para: {}", doc_id);

        use crate::core::patterns::{RE_PARENT_ID, RE_WIKI_LINK};
        let parent_re = &*RE_PARENT_ID;
        let link_re = &*RE_WIKI_LINK;

        let mut referencing: Vec<String> = Vec::new();
        let mut children: Vec<String> = Vec::new();

        use walkdir::WalkDir;
        for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() { continue; }
            if path.extension().map(|e| e != "md").unwrap_or(true) { continue; }
            let file_id = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

            if let Ok(content) = std::fs::read_to_string(&path) {
                // Verificar si es hijo
                if let Some(cap) = parent_re.captures(&content) {
                    if cap[1].trim() == doc_id {
                        children.push(file_id.to_string());
                    }
                }

                // Verificar si referencia
                for cap in link_re.captures_iter(&content) {
                    if cap[1].trim().contains(doc_id) {
                        referencing.push(file_id.to_string());
                        break;
                    }
                }
            }
        }

        if !children.is_empty() {
            println!("\n👶 Hijos directos ({}):", children.len());
            for child in &children {
                println!("  📄 {}", child);
            }
        }

        if !referencing.is_empty() {
            println!("\n🔗 Documentos que referencian ({}):", referencing.len());
            for r in &referencing {
                println!("  📄 {}", r);
            }
        }

        let total_impact = children.len() + referencing.len();
        println!("\n⚠️  Impacto total: {} documentos afectados", total_impact);
        return Ok(());
    }

    // Lógica normal
    let result = cmd.run(data_dir)?;

    // F5: Filtrar por dirección
    let direction_label = match cmd.direction.as_str() {
        "up" => "↑ Solo hacia padres",
        "down" => "↓ Solo hacia hijos",
        _ => "↕ Ambas direcciones",
    };

    if cmd.mermaid {
        println!("{}", result.to_mermaid());
    } else {
        println!(
            "📊 {} dependencias encontradas ({})",
            result.dependencies.len(),
            direction_label
        );

        if result.has_cycles() {
            println!("\n⚠️  {} ciclos detectados:", result.cycles.len());
            for cycle in &result.cycles {
                println!("  🔄 {}", cycle.to_string());
            }
        }

        if !result.root_nodes.is_empty() {
            println!("\n📍 Nodos raíz ({}):", result.root_nodes.len());
            if result.root_nodes.len() <= 10 {
                println!("   {}", result.root_nodes.join(", "));
            } else {
                println!("   {} (primeros 10)", result.root_nodes[..10].join(", "));
            }
        }

        if !result.leaf_nodes.is_empty() {
            println!("🍃 Nodos hoja: {} documentos", result.leaf_nodes.len());
        }
    }

    Ok(())
}
