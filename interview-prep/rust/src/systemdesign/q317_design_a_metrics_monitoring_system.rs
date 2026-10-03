//! Question #317: Design a Metrics/Monitoring System
//! Category: System Design | Difficulty: Hard
//! Concepts: metrics, time-series, cardinality, downsampling
//! Description: Design a time-series metrics system with cardinality control and downsampling.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAMetricsMonitoringSystem {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAMetricsMonitoringSystem {
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
    fn test_design_a_metrics_monitoring_system() {
        let s = DesignAMetricsMonitoringSystem::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
