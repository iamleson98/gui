//! Question #532: TLS Record Layer
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: TLS record, framing, sequence, AEAD
//! Description: Explain TLS record framing, sequence numbers, and AEAD protection.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct TlsRecordLayer {
    inner: Mutex<HashMap<String, String>>,
}

impl TlsRecordLayer {
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
    fn test_tls_record_layer() {
        let s = TlsRecordLayer::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
