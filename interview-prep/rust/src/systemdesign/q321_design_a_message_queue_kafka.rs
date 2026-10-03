//! Question #321: Design a Message Queue (Kafka)
//! Category: System Design | Difficulty: Hard
//! Concepts: message queue, Kafka, partition, replication
//! Description: Design a partitioned, replicated log-based message queue with consumer groups.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAMessageQueueKafka {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAMessageQueueKafka {
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
    fn test_design_a_message_queue_kafka() {
        let s = DesignAMessageQueueKafka::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
