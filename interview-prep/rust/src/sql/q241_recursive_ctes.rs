//! Question #241: Recursive CTEs
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: recursive CTE, hierarchy, traversal, anchor
//! Description: Model hierarchies and graph traversals with recursive CTEs using an anchor and a recursive member.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RecursiveCtes {
    inner: Mutex<HashMap<String, String>>,
}

impl RecursiveCtes {
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
    fn test_recursive_ctes() {
        let s = RecursiveCtes::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
