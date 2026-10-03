//! Question #523: QUIC Protocol
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: QUIC, UDP, 0-RTT, streams
//! Description: Explain QUIC's UDP-based streams, 0-RTT, and transport-level encryption.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct QuicProtocol {
    inner: Mutex<HashMap<String, String>>,
}

impl QuicProtocol {
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
    fn test_quic_protocol() {
        let s = QuicProtocol::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
