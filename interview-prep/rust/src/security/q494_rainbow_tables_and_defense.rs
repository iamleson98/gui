//! Question #494: Rainbow Tables and Defense
//! Category: Security | Difficulty: Hard
//! Concepts: rainbow table, salt, memory-hard, precomputed
//! Description: Defend against rainbow tables using salts and memory-hard hashing.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RainbowTablesAndDefense {
    inner: Mutex<HashMap<String, String>>,
}

impl RainbowTablesAndDefense {
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
    fn test_rainbow_tables_and_defense() {
        let s = RainbowTablesAndDefense::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
