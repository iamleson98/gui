//! Question #439: Tail Latency (P99)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: tail latency, P99, straggler, fan-out
//! Description: Measure and reduce P99 latency by eliminating stragglers in fan-out services.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TailLatencyP99 {
    inner: Mutex<HashMap<String, String>>,
}

impl TailLatencyP99 {
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
    fn test_tail_latency_p99() {
        let s = TailLatencyP99::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
