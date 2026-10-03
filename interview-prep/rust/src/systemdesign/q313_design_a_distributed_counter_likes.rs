//! Question #313: Design a Distributed Counter (Likes)
//! Category: System Design | Difficulty: Hard
//! Concepts: counter, CRDT, sharded, eventual
//! Description: Design an eventually consistent like counter using CRDTs and sharded counts.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignADistributedCounterLikes {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignADistributedCounterLikes {
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
    fn test_design_a_distributed_counter_likes() {
        let s = DesignADistributedCounterLikes::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
