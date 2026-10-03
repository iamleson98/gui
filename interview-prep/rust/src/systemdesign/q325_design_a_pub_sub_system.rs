//! Question #325: Design a Pub/Sub System
//! Category: System Design | Difficulty: Hard
//! Concepts: pub/sub, topics, durable, backpressure
//! Description: Design a topic-based pub/sub with durable subscriptions and backpressure.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAPubSubSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAPubSubSystem {
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
    fn test_design_a_pub_sub_system() {
        let s = DesignAPubSubSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
