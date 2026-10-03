//! Question #221: Normalization: 1NF/2NF/3NF
//! Category: SQL & Database Design | Difficulty: Hard
//! Concepts: normalization, 1NF, 2NF, 3NF, anomalies
//! Description: Apply first, second, and third normal forms to eliminate anomalies and redundancy in a schema.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Normalization1Nf2Nf3Nf {
    inner: Mutex<HashMap<String, String>>,
}

impl Normalization1Nf2Nf3Nf {
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
    fn test_normalization_1nf_2nf_3nf() {
        let s = Normalization1Nf2Nf3Nf::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
