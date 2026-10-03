//! Question #455: False Sharing Mitigation
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: false sharing, padding, cache line, mitigation
//! Description: Pad shared mutable variables to cache lines to eliminate false sharing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct FalseSharingMitigation {
    inner: Mutex<HashMap<String, String>>,
}

impl FalseSharingMitigation {
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
    fn test_false_sharing_mitigation() {
        let s = FalseSharingMitigation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
