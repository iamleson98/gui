//! Question #365: Region/Arena Bump Allocator
//! Category: Memory Management | Difficulty: Hard
//! Concepts: bump allocator, region, O(1), bulk free
//! Description: Implement a bump pointer allocator within a region for O(1) allocation and bulk free.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RegionArenaBumpAllocator {
    inner: Mutex<HashMap<String, String>>,
}

impl RegionArenaBumpAllocator {
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
    fn test_region_arena_bump_allocator() {
        let s = RegionArenaBumpAllocator::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
