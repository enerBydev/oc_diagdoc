//! Motor de grafos para dependencias y relaciones jerárquicas.
//!
//! Proporciona:
//! - Construcción de grafos de dependencias
//! - Detección de ciclos
//! - Análisis de huérfanos y jerarquía

use crate::types::DocumentId;
use std::collections::{HashMap, HashSet, VecDeque};

/// Nodo en el grafo de dependencias.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// ID del documento.
    pub id: DocumentId,
    /// IDs de documentos que este nodo referencia (outgoing).
    pub links_to: HashSet<DocumentId>,
    /// IDs de documentos que referencian a este nodo (incoming).
    pub linked_from: HashSet<DocumentId>,
    /// ID del padre jerárquico.
    pub parent: Option<DocumentId>,
    /// IDs de hijos jerárquicos.
    pub children: HashSet<DocumentId>,
}

impl GraphNode {
    pub fn new(id: DocumentId) -> Self {
        Self {
            id,
            links_to: HashSet::new(),
            linked_from: HashSet::new(),
            parent: None,
            children: HashSet::new(),
        }
    }
}

/// Grafo de dependencias de documentos.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    nodes: HashMap<DocumentId, GraphNode>,
}

impl DependencyGraph {
    /// Crea un grafo vacío.
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega un documento al grafo.
    pub fn add_node(&mut self, id: DocumentId) {
        self.nodes
            .entry(id.clone())
            .or_insert_with(|| GraphNode::new(id));
    }

    /// Agrega un documento con su padre.
    pub fn add_node_with_parent(&mut self, id: DocumentId, parent: Option<DocumentId>) {
        self.add_node(id.clone());

        if let Some(parent_id) = parent {
            self.add_node(parent_id.clone());

            // Establecer relación padre-hijo
            if let Some(node) = self.nodes.get_mut(&id) {
                node.parent = Some(parent_id.clone());
            }
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.insert(id);
            }
        }
    }

    /// Agrega un enlace entre documentos.
    pub fn add_link(&mut self, from: DocumentId, to: DocumentId) {
        self.add_node(from.clone());
        self.add_node(to.clone());

        if let Some(from_node) = self.nodes.get_mut(&from) {
            from_node.links_to.insert(to.clone());
        }
        if let Some(to_node) = self.nodes.get_mut(&to) {
            to_node.linked_from.insert(from);
        }
    }

    /// Obtiene hijos directos de un nodo.
    pub fn get_children(&self, id: &DocumentId) -> Vec<&DocumentId> {
        self.nodes
            .get(id)
            .map(|n| n.children.iter().collect())
            .unwrap_or_default()
    }

    /// Obtiene todos los descendientes (recursivo).
    pub fn get_descendants(&self, id: &DocumentId) -> Vec<DocumentId> {
        let mut result = Vec::new();
        let mut queue: VecDeque<DocumentId> = VecDeque::new();

        // Agregar hijos iniciales
        if let Some(node) = self.nodes.get(id) {
            queue.extend(node.children.iter().cloned());
        }

        while let Some(current) = queue.pop_front() {
            if !result.contains(&current) {
                result.push(current.clone());
                if let Some(node) = self.nodes.get(&current) {
                    queue.extend(node.children.iter().cloned());
                }
            }
        }

        result
    }

    /// Obtiene ancestros (recursivo hacia arriba).
    pub fn get_ancestors(&self, id: &DocumentId) -> Vec<DocumentId> {
        let mut result = Vec::new();
        let mut current = id.clone();

        while let Some(node) = self.nodes.get(&current) {
            if let Some(parent) = &node.parent {
                result.push(parent.clone());
                current = parent.clone();
            } else {
                break;
            }
        }

        result
    }

    /// Detecta ciclos en el grafo de enlaces.
    pub fn detect_cycles(&self) -> Vec<Vec<DocumentId>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for id in self.nodes.keys() {
            if !visited.contains(id) {
                let mut path = Vec::new();
                self.detect_cycles_dfs(id, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn detect_cycles_dfs(
        &self,
        id: &DocumentId,
        visited: &mut HashSet<DocumentId>,
        rec_stack: &mut HashSet<DocumentId>,
        path: &mut Vec<DocumentId>,
        cycles: &mut Vec<Vec<DocumentId>>,
    ) {
        visited.insert(id.clone());
        rec_stack.insert(id.clone());
        path.push(id.clone());

        if let Some(node) = self.nodes.get(id) {
            for neighbor in &node.links_to {
                if !visited.contains(neighbor) {
                    self.detect_cycles_dfs(neighbor, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Encontramos un ciclo
                    if let Some(start) = path.iter().position(|x| x == neighbor) {
                        let cycle: Vec<DocumentId> = path[start..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(id);
    }

    /// Encuentra documentos huérfanos (sin padre y no son master).
    pub fn orphans(&self) -> Vec<&DocumentId> {
        self.nodes
            .values()
            .filter(|n| n.parent.is_none() && !n.id.is_master())
            .map(|n| &n.id)
            .collect()
    }

    /// Encuentra documentos raíz (sin padre).
    pub fn roots(&self) -> Vec<&DocumentId> {
        self.nodes
            .values()
            .filter(|n| n.parent.is_none())
            .map(|n| &n.id)
            .collect()
    }

    /// Encuentra hojas (sin hijos).
    pub fn leaves(&self) -> Vec<&DocumentId> {
        self.nodes
            .values()
            .filter(|n| n.children.is_empty())
            .map(|n| &n.id)
            .collect()
    }

    /// Cuenta total de nodos.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Cuenta total de enlaces.
    pub fn edge_count(&self) -> usize {
        self.nodes.values().map(|n| n.links_to.len()).sum()
    }

    /// Obtiene nodos que referencian a un documento.
    pub fn get_backlinks(&self, id: &DocumentId) -> Vec<&DocumentId> {
        self.nodes
            .get(id)
            .map(|n| n.linked_from.iter().collect())
            .unwrap_or_default()
    }

    /// Genera representación Mermaid del grafo.
    pub fn to_mermaid(&self) -> String {
        let mut lines = vec!["graph TD".to_string()];

        for node in self.nodes.values() {
            // Enlaces jerárquicos (padre-hijo)
            for child in &node.children {
                lines.push(format!("    {} --> {}", node.id, child));
            }

            // Enlaces de dependencia (punteados)
            for link in &node.links_to {
                lines.push(format!("    {} -.-> {}", node.id, link));
            }
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str) -> DocumentId {
        s.parse().unwrap()
    }

    #[test]
    fn test_add_nodes() {
        let mut graph = DependencyGraph::new();
        graph.add_node(id("1"));
        graph.add_node(id("1.1"));

        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_parent_child_relationship() {
        let mut graph = DependencyGraph::new();
        graph.add_node_with_parent(id("1.1"), Some(id("1")));
        graph.add_node_with_parent(id("1.2"), Some(id("1")));

        let children = graph.get_children(&id("1"));
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_descendants() {
        let mut graph = DependencyGraph::new();
        graph.add_node_with_parent(id("1.1"), Some(id("1")));
        graph.add_node_with_parent(id("1.1.1"), Some(id("1.1")));
        graph.add_node_with_parent(id("1.1.2"), Some(id("1.1")));

        let descendants = graph.get_descendants(&id("1"));
        assert_eq!(descendants.len(), 3);
    }

    #[test]
    fn test_ancestors() {
        let mut graph = DependencyGraph::new();
        graph.add_node_with_parent(id("1.1"), Some(id("1")));
        graph.add_node_with_parent(id("1.1.1"), Some(id("1.1")));

        let ancestors = graph.get_ancestors(&id("1.1.1"));
        assert_eq!(ancestors.len(), 2);
        assert!(ancestors.contains(&id("1.1")));
        assert!(ancestors.contains(&id("1")));
    }

    #[test]
    fn test_detect_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_link(id("1"), id("2"));
        graph.add_link(id("2"), id("3"));
        graph.add_link(id("3"), id("1")); // Ciclo!

        let cycles = graph.detect_cycles();
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_no_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_link(id("1"), id("2"));
        graph.add_link(id("2"), id("3"));

        let cycles = graph.detect_cycles();
        assert!(cycles.is_empty());
    }

    #[test]
    fn test_orphans() {
        let mut graph = DependencyGraph::new();
        graph.add_node(id("0")); // Master, no es huérfano
        graph.add_node(id("5")); // Huérfano (no es master, sin padre)
        graph.add_node_with_parent(id("1.1"), Some(id("1")));
        // Nota: id("1") también es huérfano porque no tiene padre definido

        let orphans = graph.orphans();
        // "5" y "1" son huérfanos (sin padre y no son master)
        assert_eq!(orphans.len(), 2);
    }

    #[test]
    fn test_to_mermaid() {
        let mut graph = DependencyGraph::new();
        graph.add_node_with_parent(id("1.1"), Some(id("1")));
        graph.add_link(id("1"), id("2"));

        let mermaid = graph.to_mermaid();
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("1 --> 1.1"));
    }
}
