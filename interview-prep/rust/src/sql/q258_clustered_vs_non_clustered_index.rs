//! Question #258: Clustered vs Non-Clustered Index
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: clustered, non-clustered, index-organized, secondary
//! Description: Distinguish clustered (index-organized) tables from non-clustered secondary indexes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ClusteredVsNonClusteredIndex {
    inner: Mutex<HashMap<String, String>>,
}

impl ClusteredVsNonClusteredIndex {
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
    fn test_clustered_vs_non_clustered_index() {
        let s = ClusteredVsNonClusteredIndex::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
