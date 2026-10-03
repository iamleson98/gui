//! Question #256: Index-Only Scans and Visibility Map
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: index-only scan, visibility map, vacuum, heap fetch
//! Description: Explain how visibility maps enable index-only scans and the cost of vacuuming to maintain them.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IndexOnlyScansAndVisibilityMap {
    inner: Mutex<HashMap<String, String>>,
}

impl IndexOnlyScansAndVisibilityMap {
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
    fn test_index_only_scans_and_visibility_map() {
        let s = IndexOnlyScansAndVisibilityMap::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
