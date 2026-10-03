//! Question #376: G1 GC
//! Category: Memory Management | Difficulty: Hard
//! Concepts: G1, regions, pause prediction, compaction
//! Description: Explain the Garbage-First collector's region-based layout and pause-time predictability.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct G1Gc {
    inner: Mutex<HashMap<String, String>>,
}

impl G1Gc {
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
    fn test_g1_gc() {
        let s = G1Gc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
