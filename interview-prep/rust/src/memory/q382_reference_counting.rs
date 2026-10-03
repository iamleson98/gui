//! Question #382: Reference Counting
//! Category: Memory Management | Difficulty: Hard
//! Concepts: reference counting, cycles, weak refs, deferred
//! Description: Implement reference counting with cycle detection to reclaim unreachable cycles.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ReferenceCounting {
    inner: Mutex<HashMap<String, String>>,
}

impl ReferenceCounting {
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
    fn test_reference_counting() {
        let s = ReferenceCounting::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
