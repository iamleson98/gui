//! Question #288: Design a Key-Value Store
//! Category: System Design | Difficulty: Hard
//! Concepts: key-value store, consistent hashing, replication, quorum
//! Description: Design a distributed key-value store with consistent hashing, replication, and tunable consistency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DesignAKeyValueStore {
    inner: Mutex<HashMap<String, String>>,
}

impl DesignAKeyValueStore {
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
    fn test_design_a_key_value_store() {
        let s = DesignAKeyValueStore::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
