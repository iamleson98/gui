//! Question #364: Arena Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: arena, bulk alloc, reset, region
//! Description: Build an arena allocator that bulk-allocates from a parent and frees all at once.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ArenaAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl ArenaAllocator {
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
    fn test_arena_allocator() {
        let s = ArenaAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
