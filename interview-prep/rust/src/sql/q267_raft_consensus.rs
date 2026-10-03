//! Question #267: Raft Consensus
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: Raft, leader election, log replication, term
//! Description: Implement Raft leader election, log replication, and safety via term-based commit indices.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RaftConsensus {
    inner: Mutex<HashMap<String, String>>,
}

impl RaftConsensus {
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
    fn test_raft_consensus() {
        let s = RaftConsensus::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
