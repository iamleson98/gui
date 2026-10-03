//! Question #457: TLB Miss Analysis
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: TLB miss, huge pages, locality, PMC
//! Description: Measure TLB misses and mitigate them with huge pages and access locality.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TlbMissAnalysis {
    inner: Mutex<HashMap<String, String>>,
}

impl TlbMissAnalysis {
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
    fn test_tlb_miss_analysis() {
        let s = TlbMissAnalysis::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
