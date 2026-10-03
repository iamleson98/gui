//! Question #413: perf / perf_events
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: perf, perf_events, PMU, counters
//! Description: Use perf to capture hardware counters, branch misses, and cache misses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PerfPerfEvents {
    inner: Mutex<HashMap<String, String>>,
}

impl PerfPerfEvents {
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
    fn test_perf_perf_events() {
        let s = PerfPerfEvents::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
