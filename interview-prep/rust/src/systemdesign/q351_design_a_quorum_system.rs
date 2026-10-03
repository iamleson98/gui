//! Question #351: Design a Quorum System
//! Category: System Design | Difficulty: Hard
//! Concepts: quorum, R/W, consistency, latency
//! Description: Design tunable R/W quorums trading consistency for latency and availability.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAQuorumSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAQuorumSystem {
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
    fn test_design_a_quorum_system() {
        let s = DesignAQuorumSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
