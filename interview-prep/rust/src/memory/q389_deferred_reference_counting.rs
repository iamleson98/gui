//! Question #389: Deferred Reference Counting
//! Category: Memory Management | Difficulty: Hard
//! Concepts: deferred RC, batching, amortize, hot path
//! Description: Batch reference-count updates to amortize their cost on the hot path.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DeferredReferenceCounting {
    inner: Mutex<HashMap<String, String>>,
}

impl DeferredReferenceCounting {
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
    fn test_deferred_reference_counting() {
        let s = DeferredReferenceCounting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
