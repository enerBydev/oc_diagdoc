//! Iteradores personalizados para jerarquía de documentos.
//!
//! Proporciona múltiples estrategias de recorrido.

use std::collections::VecDeque;

// ═══════════════════════════════════════════════════════════════════════════
// R27: PRE-ORDER ITERATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Iterador pre-order (profundidad primero, padre antes que hijos).
#[derive(Debug)]
pub struct PreOrderIter<T> {
    stack: Vec<T>,
}

impl<T> PreOrderIter<T> {
    pub fn new(root: T) -> Self {
        Self { stack: vec![root] }
    }

    pub fn empty() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn from_vec(items: Vec<T>) -> Self {
        Self { stack: items }
    }
}

impl<T> Iterator for PreOrderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.stack.len()))
    }
}

impl<T> ExactSizeIterator for PreOrderIter<T> {}

// ═══════════════════════════════════════════════════════════════════════════
// R28: POST-ORDER ITERATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Iterador post-order (profundidad primero, hijos antes que padre).
#[derive(Debug)]
pub struct PostOrderIter<T> {
    stack: Vec<T>,
    output: Vec<T>,
    built: bool,
}

impl<T> PostOrderIter<T> {
    pub fn new(root: T) -> Self {
        Self {
            stack: vec![root],
            output: Vec::new(),
            built: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            stack: Vec::new(),
            output: Vec::new(),
            built: true,
        }
    }

    /// Pre-construye el output (para tipos que necesitan procesamiento).
    pub fn with_output(output: Vec<T>) -> Self {
        Self {
            stack: Vec::new(),
            output,
            built: true,
        }
    }
}

impl<T> Iterator for PostOrderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.built {
            // En post-order simple, primero vaciamos el stack al output
            while let Some(item) = self.stack.pop() {
                self.output.push(item);
            }
            self.built = true;
        }
        self.output.pop()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// R29: LEVEL-ORDER ITERATOR (BFS)
// ═══════════════════════════════════════════════════════════════════════════

/// Iterador level-order (amplitud primero, nivel por nivel).
#[derive(Debug)]
pub struct LevelOrderIter<T> {
    queue: VecDeque<T>,
}

impl<T> LevelOrderIter<T> {
    pub fn new(root: T) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        Self { queue }
    }

    pub fn empty() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn from_vec(items: Vec<T>) -> Self {
        Self {
            queue: items.into(),
        }
    }

    /// Agrega items al final de la cola (para expandir hijos).
    pub fn extend(&mut self, items: impl IntoIterator<Item = T>) {
        self.queue.extend(items);
    }
}

impl<T> Iterator for LevelOrderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.len(), None) // No conocemos el tamaño total
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FILTER ITERATOR WRAPPER
// ═══════════════════════════════════════════════════════════════════════════

/// Wrapper para agregar filtrado a cualquier iterador.
pub struct FilteredIter<I, F> {
    inner: I,
    predicate: F,
}

impl<I, F, T> FilteredIter<I, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    pub fn new(inner: I, predicate: F) -> Self {
        Self { inner, predicate }
    }
}

impl<I, F, T> Iterator for FilteredIter<I, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(item) if (self.predicate)(&item) => return Some(item),
                Some(_) => continue,
                None => return None,
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ITERATOR EXTENSIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Trait para extensiones de iterador.
pub trait IteratorExt: Iterator + Sized {
    /// Filtra con predicado.
    fn with_filter<F>(self, predicate: F) -> FilteredIter<Self, F>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        FilteredIter::new(self, predicate)
    }

    /// Toma los primeros N.
    fn take_n(self, n: usize) -> std::iter::Take<Self> {
        self.take(n)
    }

    /// Cuenta cuántos cumplen el predicado.
    fn count_where<F>(self, mut predicate: F) -> usize
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self.filter(|item| predicate(item)).count()
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_order_iter() {
        let iter = PreOrderIter::from_vec(vec![3, 2, 1]);
        let result: Vec<_> = iter.collect();

        assert_eq!(result, vec![1, 2, 3]); // Stack LIFO
    }

    #[test]
    fn test_pre_order_empty() {
        let iter: PreOrderIter<i32> = PreOrderIter::empty();
        assert_eq!(iter.count(), 0);
    }

    #[test]
    fn test_post_order_iter() {
        let iter = PostOrderIter::with_output(vec![1, 2, 3]);
        let result: Vec<_> = iter.collect();

        assert_eq!(result, vec![3, 2, 1]); // Reverse output
    }

    #[test]
    fn test_level_order_iter() {
        let iter = LevelOrderIter::from_vec(vec![1, 2, 3]);
        let result: Vec<_> = iter.collect();

        assert_eq!(result, vec![1, 2, 3]); // Queue FIFO
    }

    #[test]
    fn test_filtered_iter() {
        let iter = PreOrderIter::from_vec(vec![1, 2, 3, 4, 5]);
        let filtered: Vec<_> = iter.with_filter(|x| *x % 2 == 0).collect();

        assert_eq!(filtered, vec![4, 2]); // Solo pares
    }

    #[test]
    fn test_count_where() {
        let items = vec![1, 2, 3, 4, 5];
        let count = items.into_iter().count_where(|x| *x > 2);

        assert_eq!(count, 3);
    }
}
