//! Question #536: CDN Edge Caching
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: CDN, edge cache, purge, origin shield
//! Description: Use CDN edge caching with cache keys, purge, and origin shields to reduce latency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CdnEdgeCaching {
    inner: Mutex<HashMap<String, String>>,
}

impl CdnEdgeCaching {
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
    fn test_cdn_edge_caching() {
        let s = CdnEdgeCaching::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
