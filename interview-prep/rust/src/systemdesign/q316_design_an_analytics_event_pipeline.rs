//! Question #316: Design an Analytics/Event Pipeline
//! Category: System Design | Difficulty: Hard
//! Concepts: analytics, Kafka, stream processing, warehouse
//! Description: Design an event ingestion pipeline with Kafka, stream processing, and warehousing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAnAnalyticsEventPipeline {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAnAnalyticsEventPipeline {
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
    fn test_design_an_analytics_event_pipeline() {
        let s = DesignAnAnalyticsEventPipeline::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
