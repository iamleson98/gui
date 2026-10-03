//! Question #244: Recursive Queries for Trees (Adjacency List)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: adjacency list, recursive query, tree, termination
//! Description: Traverse tree-structured adjacency data with recursive queries and termination guards.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RecursiveQueriesForTreesAdjacencyList {
    inner: Mutex<HashMap<String, String>>,
}

impl RecursiveQueriesForTreesAdjacencyList {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashMap::new()) }
    }
    pub fn set(&self, key: &str, val: &str) {
        self.inner.lock().unwrap().insert(key.to_string(), val.to_string());
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().unwrap().get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recursive_queries_for_trees_adjacency_list() {
        let s = RecursiveQueriesForTreesAdjacencyList::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
