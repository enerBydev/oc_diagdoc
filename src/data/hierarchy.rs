//! Árbol jerárquico de documentos.
//!
//! Representa la estructura jerárquica de la documentación.

use std::collections::HashMap;
use crate::types::DocumentId;
use crate::data::document::Document;

// ═══════════════════════════════════════════════════════════════════════════
// TREE NODE
// ═══════════════════════════════════════════════════════════════════════════

/// Nodo en el árbol jerárquico.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// ID del documento.
    pub id: DocumentId,
    /// Documento asociado (si existe).
    pub document: Option<Document>,
    /// IDs de hijos.
    pub children: Vec<DocumentId>,
    /// ID del padre.
    pub parent: Option<DocumentId>,
    /// Profundidad en el árbol.
    pub depth: usize,
}

impl TreeNode {
    pub fn new(id: DocumentId, depth: usize) -> Self {
        Self {
            id,
            document: None,
            children: Vec::new(),
            parent: None,
            depth,
        }
    }
    
    pub fn with_document(mut self, doc: Document) -> Self {
        self.document = Some(doc);
        self
    }
    
    pub fn with_parent(mut self, parent: DocumentId) -> Self {
        self.parent = Some(parent);
        self
    }
    
    /// ¿Es una hoja? (sin hijos)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    
    /// ¿Es la raíz? (sin padre)
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
    
    /// Número de hijos.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HIERARCHY TREE
// ═══════════════════════════════════════════════════════════════════════════

/// Árbol jerárquico de documentos.
#[derive(Debug, Default)]
pub struct HierarchyTree {
    nodes: HashMap<DocumentId, TreeNode>,
    root: Option<DocumentId>,
}

impl HierarchyTree {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Construye árbol desde documentos.
    pub fn from_documents(docs: Vec<Document>) -> Self {
        let mut tree = Self::new();
        
        // Primer paso: crear todos los nodos
        for doc in &docs {
            if let Ok(id) = doc.id() {
                let depth = id.depth();
                let mut node = TreeNode::new(id.clone(), depth);
                node.document = Some(doc.clone());
                
                if let Some(parent_id) = id.parent() {
                    node.parent = Some(parent_id);
                }
                
                tree.nodes.insert(id, node);
            }
        }
        
        // Segundo paso: establecer relaciones padre-hijo
        let ids: Vec<DocumentId> = tree.nodes.keys().cloned().collect();
        for id in ids {
            if let Some(parent_id) = tree.nodes.get(&id).and_then(|n| n.parent.clone()) {
                if let Some(parent_node) = tree.nodes.get_mut(&parent_id) {
                    parent_node.children.push(id);
                }
            }
        }
        
        // Establecer raíz
        tree.root = tree.nodes.iter()
            .find(|(_, n)| n.is_root())
            .map(|(id, _)| id.clone());
        
        tree
    }
    
    /// Obtiene un nodo.
    pub fn get(&self, id: &DocumentId) -> Option<&TreeNode> {
        self.nodes.get(id)
    }
    
    /// Obtiene un nodo mutable.
    pub fn get_mut(&mut self, id: &DocumentId) -> Option<&mut TreeNode> {
        self.nodes.get_mut(id)
    }
    
    /// Raíz del árbol.
    pub fn root(&self) -> Option<&TreeNode> {
        self.root.as_ref().and_then(|id| self.nodes.get(id))
    }
    
    /// Número de nodos.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    
    /// ¿Está vacío?
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Hijos de un nodo.
    pub fn children(&self, id: &DocumentId) -> Vec<&TreeNode> {
        self.nodes.get(id)
            .map(|n| {
                n.children.iter()
                    .filter_map(|cid| self.nodes.get(cid))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Subárbol desde un nodo.
    pub fn subtree(&self, id: &DocumentId) -> Vec<&TreeNode> {
        let mut result = Vec::new();
        let mut stack = vec![id.clone()];
        
        while let Some(current) = stack.pop() {
            if let Some(node) = self.nodes.get(&current) {
                result.push(node);
                stack.extend(node.children.iter().cloned());
            }
        }
        
        result
    }
    
    /// Ancestros de un nodo.
    pub fn ancestors(&self, id: &DocumentId) -> Vec<&TreeNode> {
        let mut result = Vec::new();
        let mut current = id.clone();
        
        while let Some(node) = self.nodes.get(&current) {
            if let Some(parent_id) = &node.parent {
                if let Some(parent) = self.nodes.get(parent_id) {
                    result.push(parent);
                    current = parent_id.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        result
    }
    
    /// Recorrido preorder.
    pub fn walk_preorder(&self) -> Vec<&TreeNode> {
        let mut result = Vec::new();
        if let Some(root_id) = &self.root {
            self.walk_preorder_recursive(root_id, &mut result);
        }
        result
    }
    
    fn walk_preorder_recursive<'a>(&'a self, id: &DocumentId, result: &mut Vec<&'a TreeNode>) {
        if let Some(node) = self.nodes.get(id) {
            result.push(node);
            for child_id in &node.children {
                self.walk_preorder_recursive(child_id, result);
            }
        }
    }
    
    /// Recorrido postorder.
    pub fn walk_postorder(&self) -> Vec<&TreeNode> {
        let mut result = Vec::new();
        if let Some(root_id) = &self.root {
            self.walk_postorder_recursive(root_id, &mut result);
        }
        result
    }
    
    fn walk_postorder_recursive<'a>(&'a self, id: &DocumentId, result: &mut Vec<&'a TreeNode>) {
        if let Some(node) = self.nodes.get(id) {
            for child_id in &node.children {
                self.walk_postorder_recursive(child_id, result);
            }
            result.push(node);
        }
    }
    
    /// Hojas del árbol.
    pub fn leaves(&self) -> Vec<&TreeNode> {
        self.nodes.values().filter(|n| n.is_leaf()).collect()
    }
    
    /// Profundidad máxima.
    pub fn max_depth(&self) -> usize {
        self.nodes.values().map(|n| n.depth).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str) -> DocumentId {
        s.parse().unwrap()
    }

    #[test]
    fn test_tree_node() {
        let node = TreeNode::new(id("1.1"), 2);
        assert!(node.is_leaf());
        assert!(node.is_root());
    }

    #[test]
    fn test_hierarchy_basic() {
        let mut tree = HierarchyTree::new();
        tree.nodes.insert(id("0"), TreeNode::new(id("0"), 1));
        tree.nodes.insert(id("1"), TreeNode::new(id("1"), 1).with_parent(id("0")));
        tree.root = Some(id("0"));
        
        // Agregar hijo
        if let Some(root) = tree.nodes.get_mut(&id("0")) {
            root.children.push(id("1"));
        }
        
        assert_eq!(tree.len(), 2);
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_children() {
        let mut tree = HierarchyTree::new();
        let mut root = TreeNode::new(id("0"), 1);
        root.children = vec![id("1"), id("2")];
        tree.nodes.insert(id("0"), root);
        tree.nodes.insert(id("1"), TreeNode::new(id("1"), 1));
        tree.nodes.insert(id("2"), TreeNode::new(id("2"), 1));
        tree.root = Some(id("0"));
        
        let children = tree.children(&id("0"));
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_leaves() {
        let mut tree = HierarchyTree::new();
        let mut root = TreeNode::new(id("0"), 1);
        root.children = vec![id("1")];
        tree.nodes.insert(id("0"), root);
        tree.nodes.insert(id("1"), TreeNode::new(id("1"), 1));
        
        let leaves = tree.leaves();
        assert_eq!(leaves.len(), 1);
        assert_eq!(leaves[0].id, id("1"));
    }
}
