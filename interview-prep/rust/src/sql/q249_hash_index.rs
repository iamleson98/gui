//! Question #249: Hash Index
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: hash index, equality, no range, buckets
//! Description: Use hash indexes for equality lookups and note their unsuitability for range queries.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HashIndex {
    inner: Mutex<HashMap<String, String>>,
}

impl HashIndex {
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
    fn test_hash_index() {
        let s = HashIndex::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
