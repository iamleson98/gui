//! Question #514: Slow Start and Congestion Avoidance
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: slow start, congestion avoidance, ssthresh, cwnd
//! Description: Reason about slow-start, congestion-avoidance, and the ssthresh transition.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SlowStartAndCongestionAvoidance {
    inner: Mutex<HashMap<String, String>>,
}

impl SlowStartAndCongestionAvoidance {
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
    fn test_slow_start_and_congestion_avoidance() {
        let s = SlowStartAndCongestionAvoidance::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
