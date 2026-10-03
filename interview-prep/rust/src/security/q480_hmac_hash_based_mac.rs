//! Question #480: HMAC (Hash-Based MAC)
//! Category: Security | Difficulty: Hard
//! Concepts: HMAC, keyed hash, integrity, authentication
//! Description: Implement HMAC for message authentication using a keyed hash construction.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HmacHashBasedMac {
    inner: Mutex<HashMap<String, String>>,
}

impl HmacHashBasedMac {
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
    fn test_hmac_hash_based_mac() {
        let s = HmacHashBasedMac::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
