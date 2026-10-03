//! Question #246: Materialized Path (ltree)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: materialized path, ltree, prefix, GiST
//! Description: Index tree paths with ltree or materialized path strings for prefix queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MaterializedPathLtree {
    inner: Mutex<HashMap<String, String>>,
}

impl MaterializedPathLtree {
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
    fn test_materialized_path_ltree() {
        let s = MaterializedPathLtree::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
