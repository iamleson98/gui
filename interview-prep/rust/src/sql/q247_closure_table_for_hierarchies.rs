//! Question #247: Closure Table for Hierarchies
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: closure table, hierarchy, ancestor, descendant
//! Description: Store all ancestor-descendant pairs in a closure table for fast descendant and depth queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ClosureTableForHierarchies {
    inner: Mutex<HashMap<String, String>>,
}

impl ClosureTableForHierarchies {
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
    fn test_closure_table_for_hierarchies() {
        let s = ClosureTableForHierarchies::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
