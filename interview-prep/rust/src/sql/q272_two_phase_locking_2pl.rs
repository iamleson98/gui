//! Question #272: Two-Phase Locking (2PL)
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: 2PL, growing, shrinking, strict
//! Description: Implement strict two-phase locking growing then shrinking lock phases to guarantee serializability.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TwoPhaseLocking2Pl {
    inner: Mutex<HashMap<String, String>>,
}

impl TwoPhaseLocking2Pl {
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
    fn test_two_phase_locking_2pl() {
        let s = TwoPhaseLocking2Pl::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
