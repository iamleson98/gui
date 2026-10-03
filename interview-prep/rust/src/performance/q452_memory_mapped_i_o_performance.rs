//! Question #452: Memory-Mapped I/O Performance
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: mmap, TLB, page fault, performance
//! Description: Evaluate mmap-backed I/O performance and TLB/page-fault tradeoffs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryMappedIOPerformance {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryMappedIOPerformance {
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
    fn test_memory_mapped_i_o_performance() {
        let s = MemoryMappedIOPerformance::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
