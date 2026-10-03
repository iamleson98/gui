//! Question #383: Tracing vs Reference Counting
//! Category: Memory Management | Difficulty: Hard
//! Concepts: tracing, reference counting, pause, throughput
//! Description: Contrast tracing and reference counting collectors on pause time and throughput.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TracingVsReferenceCounting {
    inner: Mutex<HashMap<String, String>>,
}

impl TracingVsReferenceCounting {
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
    fn test_tracing_vs_reference_counting() {
        let s = TracingVsReferenceCounting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
