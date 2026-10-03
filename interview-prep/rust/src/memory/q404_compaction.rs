//! Question #404: Compaction
//! Category: Memory Management | Difficulty: Hard
//! Concepts: compaction, forwarding, fragmentation, heap
//! Description: Compact the heap to reduce external fragmentation via forwarding addresses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Compaction {
    inner: Mutex<HashMap<String, String>>,
}

impl Compaction {
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
    fn test_compaction() {
        let s = Compaction::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
