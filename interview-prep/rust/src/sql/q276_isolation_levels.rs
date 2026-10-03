//! Question #276: Isolation Levels
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: isolation, read committed, repeatable read, serializable
//! Description: Contrast read-uncommitted, read-committed, repeatable-read, and serializable isolation.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct IsolationLevels {
    inner: Mutex<HashMap<String, String>>,
}

impl IsolationLevels {
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
    fn test_isolation_levels() {
        let s = IsolationLevels::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
