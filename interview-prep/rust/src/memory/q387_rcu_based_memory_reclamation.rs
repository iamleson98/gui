//! Question #387: RCU-Based Memory Reclamation
//! Category: Memory Management | Difficulty: Hard
//! Concepts: RCU, grace period, reclamation, readers
//! Description: Reclaim nodes after a grace period so concurrent readers see consistent state.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RcuBasedMemoryReclamation {
    inner: Mutex<HashMap<String, String>>,
}

impl RcuBasedMemoryReclamation {
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
    fn test_rcu_based_memory_reclamation() {
        let s = RcuBasedMemoryReclamation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
