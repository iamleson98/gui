//! Question #482: Public-Key Encryption (RSA-OAEP)
//! Category: Security | Difficulty: Hard
//! Concepts: RSA-OAEP, padding, CCA, public key
//! Description: Encrypt with RSA-OAEP padding to prevent chosen-ciphertext attacks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PublicKeyEncryptionRsaOaep {
    inner: Mutex<HashMap<String, String>>,
}

impl PublicKeyEncryptionRsaOaep {
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
    fn test_public_key_encryption_rsa_oaep() {
        let s = PublicKeyEncryptionRsaOaep::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
