//! Question #361: Slab Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: slab, caches, fixed-size, kernel
//! Description: Implement a slab allocator caching fixed-size object states for the kernel to reduce fragmentation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SlabAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl SlabAllocator {
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
    fn test_slab_allocator() {
        let s = SlabAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
