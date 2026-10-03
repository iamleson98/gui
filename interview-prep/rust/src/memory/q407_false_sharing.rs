//! Question #407: False Sharing
//! Category: Memory Management | Difficulty: Hard
//! Concepts: false sharing, cache line, padding, contention
//! Description: Diagnose false sharing on shared mutable fields in the same cache line and pad them apart.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FalseSharing {
    inner: Mutex<HashMap<String, String>>,
}

impl FalseSharing {
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
    fn test_false_sharing() {
        let s = FalseSharing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
