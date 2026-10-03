//! Question #468: Insecure Deserialization
//! Category: Security | Difficulty: Hard
//! Concepts: deserialization, type allowlist, gadget, RCE
//! Description: Prevent insecure deserialization by avoiding native formats and enforcing type allowlists.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct InsecureDeserialization {
    inner: Mutex<HashMap<String, String>>,
}

impl InsecureDeserialization {
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
    fn test_insecure_deserialization() {
        let s = InsecureDeserialization::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
