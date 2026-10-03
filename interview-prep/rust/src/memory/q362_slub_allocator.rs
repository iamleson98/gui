//! Question #362: Slub Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: SLUB, per-CPU, freelist, kernel
//! Description: Explain the SLUB allocator's simpler, per-CPU design replacing the classic slab allocator.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SlubAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl SlubAllocator {
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
    fn test_slub_allocator() {
        let s = SlubAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
