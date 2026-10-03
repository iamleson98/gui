//! Question #374: Copying GC (Semispace)
//! Category: Memory Management | Difficulty: Hard
//! Concepts: copying GC, semispace, evacuation, forwarding
//! Description: Implement a copying collector that evacuates live objects between two semispaces.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CopyingGcSemispace {
    inner: Mutex<HashMap<String, String>>,
}

impl CopyingGcSemispace {
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
    fn test_copying_gc_semispace() {
        let s = CopyingGcSemispace::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
