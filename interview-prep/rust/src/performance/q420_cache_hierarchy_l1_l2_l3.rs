//! Question #420: Cache Hierarchy (L1/L2/L3)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: cache, L1/L2/L3, latency, bandwidth
//! Description: Model the L1/L2/L3 cache hierarchy and quantify latency and bandwidth at each level.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CacheHierarchyL1L2L3 {
    inner: Mutex<HashMap<String, String>>,
}

impl CacheHierarchyL1L2L3 {
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
    fn test_cache_hierarchy_l1_l2_l3() {
        let s = CacheHierarchyL1L2L3::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
