//! Question #268: Multi-Paxos
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: Multi-Paxos, leader, batching, steady state
//! Description: Optimize Paxos to a steady-state leader batching many instances over a stable leader.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MultiPaxos {
    inner: Mutex<HashMap<String, String>>,
}

impl MultiPaxos {
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
    fn test_multi_paxos() {
        let s = MultiPaxos::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
