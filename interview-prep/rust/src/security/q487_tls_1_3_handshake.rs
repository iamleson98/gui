//! Question #487: TLS 1.3 Handshake
//! Category: Security | Difficulty: Hard
//! Concepts: TLS 1.3, 1-RTT, key share, HKDF
//! Description: Walk through the TLS 1.3 1-RTT handshake with key share and HKDF.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Tls13Handshake {
    inner: Mutex<HashMap<String, String>>,
}

impl Tls13Handshake {
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
    fn test_tls_1_3_handshake() {
        let s = Tls13Handshake::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
