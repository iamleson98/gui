//! Question #434: Hyperthreading/SMT Throughput
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: SMT, hyperthreading, throughput, contention
//! Description: Reason about SMT throughput gains and contention on shared execution resources.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HyperthreadingSmtThroughput {
    inner: Mutex<HashMap<String, String>>,
}

impl HyperthreadingSmtThroughput {
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
    fn test_hyperthreading_smt_throughput() {
        let s = HyperthreadingSmtThroughput::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
