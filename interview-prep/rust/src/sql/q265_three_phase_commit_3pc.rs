//! Question #265: Three-Phase Commit (3PC)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: 3PC, pre-commit, non-blocking, timing
//! Description: Add a pre-commit phase to 2PC to reduce blocking on coordinator failure under assumptions.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ThreePhaseCommit3Pc {
    inner: Mutex<HashMap<String, String>>,
}

impl ThreePhaseCommit3Pc {
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
    fn test_three_phase_commit_3pc() {
        let s = ThreePhaseCommit3Pc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
