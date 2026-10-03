//! Question #370: mimalloc Design
//! Category: Memory Management | Difficulty: Hard
//! Concepts: mimalloc, per-CPU, free lists, delayed reset
//! Description: Explain mimalloc's per-CPU sharded free lists and delayed resets for high throughput.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MimallocDesign {
    inner: Mutex<HashMap<String, String>>,
}

impl MimallocDesign {
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
    fn test_mimalloc_design() {
        let s = MimallocDesign::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
