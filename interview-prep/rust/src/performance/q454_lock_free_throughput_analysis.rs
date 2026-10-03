//! Question #454: Lock-Free Throughput Analysis
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: lock-free, throughput, contention, CAS
//! Description: Analyze whether lock-free structures actually improve throughput under contention.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LockFreeThroughputAnalysis {
    inner: Mutex<HashMap<String, String>>,
}

impl LockFreeThroughputAnalysis {
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
    fn test_lock_free_throughput_analysis() {
        let s = LockFreeThroughputAnalysis::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
