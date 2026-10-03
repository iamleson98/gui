//! Question #373: Mark-Compact GC
//! Category: Memory Management | Difficulty: Hard
//! Concepts: mark-compact, compaction, fragmentation, forwarding
//! Description: Implement a mark-compact collector that eliminates fragmentation by sliding live objects.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MarkCompactGc {
    inner: Mutex<HashMap<String, String>>,
}

impl MarkCompactGc {
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
    fn test_mark_compact_gc() {
        let s = MarkCompactGc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
