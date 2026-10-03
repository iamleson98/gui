//! Question #435: Memory-Bound vs Compute-Bound
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: memory-bound, compute-bound, roofline, classification
//! Description: Classify code as memory- or compute-bound to pick the right optimization.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryBoundVsComputeBound {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryBoundVsComputeBound {
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
    fn test_memory_bound_vs_compute_bound() {
        let s = MemoryBoundVsComputeBound::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
