//! Question #421: Cache Lines and Prefetching
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: cache line, prefetch, locality, size
//! Description: Size data accesses to cache lines and exploit hardware prefetching.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CacheLinesAndPrefetching {
    inner: Mutex<HashMap<String, String>>,
}

impl CacheLinesAndPrefetching {
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
    fn test_cache_lines_and_prefetching() {
        let s = CacheLinesAndPrefetching::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
