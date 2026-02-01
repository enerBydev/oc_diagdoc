//! Trait Queryable - Para búsqueda en colecciones de documentos.

/// Resultado de búsqueda.
#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub item: T,
    pub score: f64,
    pub matches: Vec<String>,
}

impl<T> QueryResult<T> {
    pub fn new(item: T, score: f64) -> Self {
        Self {
            item,
            score,
            matches: Vec::new(),
        }
    }
    
    pub fn with_matches(mut self, matches: Vec<String>) -> Self {
        self.matches = matches;
        self
    }
}

/// Trait para colecciones que pueden ser consultadas.
pub trait Queryable {
    type Item;
    
    /// Busca elementos que contengan el término.
    fn search(&self, query: &str) -> Vec<QueryResult<Self::Item>>
    where
        Self::Item: Clone;
    
    /// Filtra elementos por predicado.
    fn filter<F>(&self, predicate: F) -> Vec<Self::Item>
    where
        F: Fn(&Self::Item) -> bool,
        Self::Item: Clone;
    
    /// Encuentra el primer elemento que coincida.
    fn find_first(&self, query: &str) -> Option<Self::Item>
    where
        Self::Item: Clone,
    {
        self.search(query).into_iter().next().map(|r| r.item)
    }
    
    /// Cuenta elementos que coinciden.
    fn count_matches(&self, query: &str) -> usize
    where
        Self::Item: Clone,
    {
        self.search(query).len()
    }
}

/// Implementación para Vec de strings.
impl Queryable for Vec<String> {
    type Item = String;
    
    fn search(&self, query: &str) -> Vec<QueryResult<String>> {
        let query_lower = query.to_lowercase();
        self.iter()
            .filter(|s| s.to_lowercase().contains(&query_lower))
            .map(|s| {
                let score = if s.to_lowercase() == query_lower {
                    100.0
                } else {
                    50.0
                };
                QueryResult::new(s.clone(), score)
            })
            .collect()
    }
    
    fn filter<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&String) -> bool,
    {
        self.iter().filter(|s| predicate(s)).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search() {
        let items = vec![
            "hello world".to_string(),
            "hello rust".to_string(),
            "goodbye".to_string(),
        ];
        
        let results = items.search("hello");
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn test_filter() {
        let items = vec!["a".to_string(), "ab".to_string(), "abc".to_string()];
        let filtered = items.filter(|s| s.len() > 1);
        
        assert_eq!(filtered.len(), 2);
    }
}
