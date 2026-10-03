//! Question #348: Design a Consensus-Based Config Store
//! Category: System Design | Difficulty: Hard
//! Concepts: config store, Raft, watchers, linearizable
//! Description: Design an etcd/ZooKeeper-like config store using Raft and watchers.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAConsensusBasedConfigStore {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAConsensusBasedConfigStore {
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
    fn test_design_a_consensus_based_config_store() {
        let s = DesignAConsensusBasedConfigStore::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
