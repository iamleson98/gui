//! Question #232: Composite Primary Keys
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: composite key, primary key, indexes, foreign key
//! Description: Design composite primary keys and reason about their impact on indexes and foreign keys.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CompositePrimaryKeys {
    inner: Mutex<HashMap<String, String>>,
}

impl CompositePrimaryKeys {
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
    fn test_composite_primary_keys() {
        let s = CompositePrimaryKeys::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
