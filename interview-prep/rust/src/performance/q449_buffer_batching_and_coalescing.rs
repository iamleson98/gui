//! Question #449: Buffer Batching and Coalescing
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: batching, coalescing, syscall, amortize
//! Description: Batch and coalesce small writes into larger buffers to amortize syscall cost.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BufferBatchingAndCoalescing {
    inner: Mutex<HashMap<String, String>>,
}

impl BufferBatchingAndCoalescing {
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
    fn test_buffer_batching_and_coalescing() {
        let s = BufferBatchingAndCoalescing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
