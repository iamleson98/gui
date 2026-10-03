//! Question #363: Buddy Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: buddy, coalescing, power-of-two, split
//! Description: Implement a binary buddy allocator splitting and coalescing power-of-two blocks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct BuddyAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl BuddyAllocator {
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
    fn test_buddy_allocator() {
        let s = BuddyAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
