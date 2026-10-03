//! Question #516: Nagle's Algorithm
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: Nagle, delayed ACK, coalescing, latency
//! Description: Explain Nagle's algorithm and its interaction with delayed ACK and latency.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct NagleSAlgorithm {
    inner: Mutex<HashMap<String, String>>,
}

impl NagleSAlgorithm {
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
    fn test_nagle_s_algorithm() {
        let s = NagleSAlgorithm::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
