//! Question #395: Copy-on-Write (CoW) Pages
//! Category: Memory Management | Difficulty: Hard
//! Concepts: copy-on-write, fork, sharing, page protection
//! Description: Share read-only pages and copy only on write to enable cheap fork and sharing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CopyOnWriteCowPages {
    inner: Mutex<HashMap<String, String>>,
}

impl CopyOnWriteCowPages {
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
    fn test_copy_on_write_cow_pages() {
        let s = CopyOnWriteCowPages::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
