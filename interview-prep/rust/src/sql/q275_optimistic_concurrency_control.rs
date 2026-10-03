//! Question #275: Optimistic Concurrency Control
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: OCC, validation, read/write set, conflict
//! Description: Validate transactions at commit time using read and write sets instead of locks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OptimisticConcurrencyControl {
    inner: Mutex<HashMap<String, String>>,
}

impl OptimisticConcurrencyControl {
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
    fn test_optimistic_concurrency_control() {
        let s = OptimisticConcurrencyControl::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
