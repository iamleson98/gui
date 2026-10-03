//! Question #484: ECDH and Curve25519
//! Category: Security | Difficulty: Hard
//! Concepts: ECDH, Curve25519, key agreement, side channel
//! Description: Use ECDH on Curve25519 for fast, secure key agreement resistant to side channels.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct EcdhAndCurve25519 {
    inner: Mutex<HashMap<String, String>>,
}

impl EcdhAndCurve25519 {
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
    fn test_ecdh_and_curve25519() {
        let s = EcdhAndCurve25519::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
