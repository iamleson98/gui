//! Question #456: Cache Miss Profiling
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: cache miss, profiling, PMC, layout
//! Description: Measure L1/L2/L3 and LLC misses to find cache-unfriendly data layouts.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CacheMissProfiling {
    inner: Mutex<HashMap<String, String>>,
}

impl CacheMissProfiling {
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
    fn test_cache_miss_profiling() {
        let s = CacheMissProfiling::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
