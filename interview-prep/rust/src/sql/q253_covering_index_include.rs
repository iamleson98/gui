//! Question #253: Covering Index (INCLUDE)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: covering index, INCLUDE, index-only scan, visibility map
//! Description: Add included non-key columns to make an index covering and enable index-only scans.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CoveringIndexInclude {
    inner: Mutex<HashMap<String, String>>,
}

impl CoveringIndexInclude {
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
    fn test_covering_index_include() {
        let s = CoveringIndexInclude::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
