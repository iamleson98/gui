//! Question #322: Design Kafka Log Structure
//! Category: System Design | Difficulty: Hard
//! Concepts: Kafka log, segments, index, retention
//! Description: Explain Kafka's append-only segmented log with indexes and retention by size/time.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignKafkaLogStructure {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignKafkaLogStructure {
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
    fn test_design_kafka_log_structure() {
        let s = DesignKafkaLogStructure::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
