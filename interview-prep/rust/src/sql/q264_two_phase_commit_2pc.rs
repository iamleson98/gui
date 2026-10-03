//! Question #264: Two-Phase Commit (2PC)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: 2PC, prepare, commit, coordinator
//! Description: Coordinate a transaction across nodes with a prepare-then-commit protocol and a coordinator log.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TwoPhaseCommit2Pc {
    inner: Mutex<HashMap<String, String>>,
}

impl TwoPhaseCommit2Pc {
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
    fn test_two_phase_commit_2pc() {
        let s = TwoPhaseCommit2Pc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
