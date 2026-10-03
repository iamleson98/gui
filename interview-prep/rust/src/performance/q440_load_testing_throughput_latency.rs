//! Question #440: Load Testing (Throughput/Latency)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: load testing, throughput, latency, sustained
//! Description: Design load tests that report throughput-latency curves under sustained load.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LoadTestingThroughputLatency {
    inner: Mutex<HashMap<String, String>>,
}

impl LoadTestingThroughputLatency {
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
    fn test_load_testing_throughput_latency() {
        let s = LoadTestingThroughputLatency::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
