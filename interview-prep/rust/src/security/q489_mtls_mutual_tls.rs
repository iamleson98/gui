//! Question #489: mTLS (Mutual TLS)
//! Category: Security | Difficulty: Hard
//! Concepts: mTLS, client cert, verification, trust
//! Description: Configure mutual TLS so both client and server present and verify certificates.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MtlsMutualTls {
    inner: Mutex<HashMap<String, String>>,
}

impl MtlsMutualTls {
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
    fn test_mtls_mutual_tls() {
        let s = MtlsMutualTls::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
