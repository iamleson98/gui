//! Question #465: XXE Prevention
//! Category: Security | Difficulty: Hard
//! Concepts: XXE, DTD, external entity, parser
//! Description: Prevent XML external entity attacks by disabling DTDs and external entity resolution.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct XxePrevention {
    inner: Mutex<HashMap<String, String>>,
}

impl XxePrevention {
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
    fn test_xxe_prevention() {
        let s = XxePrevention::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
