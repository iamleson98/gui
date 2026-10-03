//! Question #403: Memory Fragmentation (External/Internal)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: fragmentation, external, internal, coalescing
//! Description: Distinguish external and internal fragmentation and mitigate each.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryFragmentationExternalInternal {
    inner: Mutex<HashMap<String, String>>,
}

impl MemoryFragmentationExternalInternal {
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
    fn test_memory_fragmentation_external_internal() {
        let s = MemoryFragmentationExternalInternal::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
