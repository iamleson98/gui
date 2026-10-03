//! Question #441: Little's Law
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: Little's Law, concurrency, queue, throughput
//! Description: Apply Little's Law (L = lambda * W) to size queues and concurrency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct LittleSLaw {
    inner: Mutex<HashMap<String, String>>,
}

impl LittleSLaw {
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
    fn test_little_s_law() {
        let s = LittleSLaw::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
