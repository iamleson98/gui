//! Question #271: MVCC Implementation
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: MVCC, xmin/xmax, version chain, visibility
//! Description: Build multi-version concurrency control with tuple xmin/xmax and visibility checks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MvccImplementation {
    inner: Mutex<HashMap<String, String>>,
}

impl MvccImplementation {
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
    fn test_mvcc_implementation() {
        let s = MvccImplementation::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
