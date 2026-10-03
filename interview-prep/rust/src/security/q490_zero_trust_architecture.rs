//! Question #490: Zero-Trust Architecture
//! Category: Security | Difficulty: Hard
//! Concepts: zero trust, per-request auth, no implicit trust, policy
//! Description: Design a zero-trust architecture authenticating every request without network trust.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ZeroTrustArchitecture {
    inner: Mutex<HashMap<String, String>>,
}

impl ZeroTrustArchitecture {
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
    fn test_zero_trust_architecture() {
        let s = ZeroTrustArchitecture::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
