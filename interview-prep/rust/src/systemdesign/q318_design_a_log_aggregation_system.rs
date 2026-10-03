//! Question #318: Design a Log Aggregation System
//! Category: System Design | Difficulty: Hard
//! Concepts: logs, ingestion, indexing, retention
//! Description: Design a log pipeline with ingestion, indexing, retention, and query.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignALogAggregationSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignALogAggregationSystem {
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
    fn test_design_a_log_aggregation_system() {
        let s = DesignALogAggregationSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
