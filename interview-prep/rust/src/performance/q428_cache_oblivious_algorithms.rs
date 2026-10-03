//! Question #428: Cache-Oblivious Algorithms
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: cache-oblivious, recursive blocking, portable, I/O
//! Description: Design algorithms that achieve cache efficiency without tuning to cache size.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CacheObliviousAlgorithms {
    inner: Mutex<HashMap<String, String>>,
}

impl CacheObliviousAlgorithms {
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
    fn test_cache_oblivious_algorithms() {
        let s = CacheObliviousAlgorithms::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
