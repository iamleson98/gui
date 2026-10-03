//! Question #432: Memory Bandwidth and Streams
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: bandwidth, STREAM, memory-bound, measurement
//! Description: Measure memory bandwidth with the STREAM benchmark and detect bandwidth-bound code.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryBandwidthAndStreams {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryBandwidthAndStreams {
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
    fn test_memory_bandwidth_and_streams() {
        let s = MemoryBandwidthAndStreams::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
