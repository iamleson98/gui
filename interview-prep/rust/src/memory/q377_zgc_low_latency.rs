//! Question #377: ZGC (Low Latency)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: ZGC, colored pointer, load barrier, low latency
//! Description: Explain ZGC's colored pointers and load barriers for sub-millisecond pauses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ZgcLowLatency {
    inner: Mutex<HashMap<String, String>>,
}

impl ZgcLowLatency {
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
    fn test_zgc_low_latency() {
        let s = ZgcLowLatency::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
