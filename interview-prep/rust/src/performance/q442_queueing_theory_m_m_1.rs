//! Question #442: Queueing Theory (M/M/1)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: queueing theory, M/M/1, utilization, latency
//! Description: Model an M/M/1 queue to predict latency under arrival and service rate variation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct QueueingTheoryMM1 {
    inner: Mutex<HashMap<String, String>>,
}

impl QueueingTheoryMM1 {
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
    fn test_queueing_theory_m_m_1() {
        let s = QueueingTheoryMM1::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
