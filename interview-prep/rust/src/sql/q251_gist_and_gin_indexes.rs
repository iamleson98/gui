//! Question #251: GiST and GIN Indexes
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: GiST, GIN, full-text, custom types
//! Description: Choose GiST vs GIN for full-text and custom data types based on query and update patterns.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct GistAndGinIndexes {
    inner: Mutex<HashMap<String, String>>,
}

impl GistAndGinIndexes {
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
    fn test_gist_and_gin_indexes() {
        let s = GistAndGinIndexes::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
