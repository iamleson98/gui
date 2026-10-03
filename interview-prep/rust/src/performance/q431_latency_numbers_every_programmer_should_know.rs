//! Question #431: Latency Numbers Every Programmer Should Know
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: latency, L1, DRAM, network
//! Description: Reason about L1, DRAM, SSD, and network latencies to design fast systems.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LatencyNumbersEveryProgrammerShouldKnow {
    inner: Mutex<HashMap<String, String>>,
}

impl LatencyNumbersEveryProgrammerShouldKnow {
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
    fn test_latency_numbers_every_programmer_should_know() {
        let s = LatencyNumbersEveryProgrammerShouldKnow::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
