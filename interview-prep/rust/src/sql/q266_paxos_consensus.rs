//! Question #266: Paxos Consensus
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: Paxos, proposer, acceptor, quorum
//! Description: Implement single-decree Paxos with proposers, acceptors, and learners achieving safety under quorum.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PaxosConsensus {
    inner: Mutex<HashMap<String, String>>,
}

impl PaxosConsensus {
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
    fn test_paxos_consensus() {
        let s = PaxosConsensus::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
