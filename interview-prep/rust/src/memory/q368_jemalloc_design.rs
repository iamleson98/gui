//! Question #368: jemalloc Design
//! Category: Memory Management | Difficulty: Hard
//! Concepts: jemalloc, size class, arena, thread cache
//! Description: Explain jemalloc's size-class bins, arenas, and thread caches for low fragmentation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct JemallocDesign {
    inner: Mutex<HashMap<String, String>>,
}

impl JemallocDesign {
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
    fn test_jemalloc_design() {
        let s = JemallocDesign::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
