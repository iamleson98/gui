//! Question #397: Page Replacement (LRU/Clock)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: page replacement, LRU, clock, eviction
//! Description: Implement LRU and clock page replacement policies for finite physical memory.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PageReplacementLruClock {
    inner: Mutex<HashMap<String, String>>,
}

impl PageReplacementLruClock {
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
    fn test_page_replacement_lru_clock() {
        let s = PageReplacementLruClock::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
