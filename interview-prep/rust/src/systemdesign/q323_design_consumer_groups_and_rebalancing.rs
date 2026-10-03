//! Question #323: Design Consumer Groups and Rebalancing
//! Category: System Design | Difficulty: Hard
//! Concepts: consumer group, rebalancing, coordinator, session
//! Description: Design consumer group coordination with rebalancing strategies and session timeouts.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignConsumerGroupsAndRebalancing {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignConsumerGroupsAndRebalancing {
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
    fn test_design_consumer_groups_and_rebalancing() {
        let s = DesignConsumerGroupsAndRebalancing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
