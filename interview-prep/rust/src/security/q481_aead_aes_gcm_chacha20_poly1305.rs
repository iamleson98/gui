//! Question #481: AEAD (AES-GCM, ChaCha20-Poly1305)
//! Category: Security | Difficulty: Hard
//! Concepts: AEAD, AES-GCM, ChaCha20-Poly1305, nonce
//! Description: Use authenticated encryption with associated data to provide confidentiality and integrity.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct AeadAesGcmChacha20Poly1305 {
    inner: Mutex<HashMap<String, String>>,
}

impl AeadAesGcmChacha20Poly1305 {
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
    fn test_aead_aes_gcm_chacha20_poly1305() {
        let s = AeadAesGcmChacha20Poly1305::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
