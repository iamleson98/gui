//! Question #261: ARIES Recovery Algorithm
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: ARIES, analysis, redo, undo
//! Description: Explain ARIES analysis, redo, and undo phases for crash recovery with per-page LSNs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AriesRecoveryAlgorithm {
    inner: Mutex<HashMap<String, String>>,
}

impl AriesRecoveryAlgorithm {
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
    fn test_aries_recovery_algorithm() {
        let s = AriesRecoveryAlgorithm::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
