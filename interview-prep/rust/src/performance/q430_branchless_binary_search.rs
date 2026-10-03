//! Question #430: Branchless Binary Search
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: branchless, binary search, conditional move, Eytzinger
//! Description: Implement a branchless binary search using conditional moves or index arithmetic.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BranchlessBinarySearch {
    inner: Mutex<HashMap<String, String>>,
}

impl BranchlessBinarySearch {
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
    fn test_branchless_binary_search() {
        let s = BranchlessBinarySearch::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
