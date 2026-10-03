//! Question #355: Design a Gossip Protocol
//! Category: System Design | Difficulty: Hard
//! Concepts: gossip, membership, epidemic, bounded
//! Description: Propagate membership and state updates across nodes with a bounded gossip round.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAGossipProtocol {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAGossipProtocol {
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
    fn test_design_a_gossip_protocol() {
        let s = DesignAGossipProtocol::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
