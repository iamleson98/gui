//! Question #289: Design a Distributed Cache
//! Category: System Design | Difficulty: Hard
//! Concepts: cache, eviction, sharding, failover
//! Description: Design a Memcached/Redis-like cache with eviction, sharding, and failover.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADistributedCache {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADistributedCache {
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
    fn test_design_a_distributed_cache() {
        let s = DesignADistributedCache::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
