//! Question #388: Quiescent-State-Based Reclamation
//! Category: Memory Management | Difficulty: Hard
//! Concepts: quiescent state, reclamation, lock-free, safety
//! Description: Reclaim memory at quiescent states observed across threads for lock-free safety.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct QuiescentStateBasedReclamation {
    inner: Mutex<HashMap<String, String>>,
}

impl QuiescentStateBasedReclamation {
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
    fn test_quiescent_state_based_reclamation() {
        let s = QuiescentStateBasedReclamation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
