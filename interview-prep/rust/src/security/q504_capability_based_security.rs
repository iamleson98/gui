//! Question #504: Capability-Based Security
//! Category: Security | Difficulty: Hard
//! Concepts: capabilities, unforgeable, delegation, ACL
//! Description: Design authorization around unforgeable capabilities rather than identity-based ACLs.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CapabilityBasedSecurity {
    inner: Mutex<HashMap<String, String>>,
}

impl CapabilityBasedSecurity {
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
    fn test_capability_based_security() {
        let s = CapabilityBasedSecurity::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
