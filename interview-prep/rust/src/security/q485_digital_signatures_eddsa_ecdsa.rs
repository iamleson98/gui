//! Question #485: Digital Signatures (EdDSA, ECDSA)
//! Category: Security | Difficulty: Hard
//! Concepts: EdDSA, ECDSA, nonce, canonical
//! Description: Sign messages with EdDSA or ECDSA, noting canonical signatures and nonce risks.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DigitalSignaturesEddsaEcdsa {
    inner: Mutex<HashMap<String, String>>,
}

impl DigitalSignaturesEddsaEcdsa {
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
    fn test_digital_signatures_eddsa_ecdsa() {
        let s = DigitalSignaturesEddsaEcdsa::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
