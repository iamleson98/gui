//! Question #491: Hashing vs Encryption vs Encoding
//! Category: Security | Difficulty: Hard
//! Concepts: hashing, encryption, encoding, purpose
//! Description: Distinguish hashing, encryption, and encoding and pick the right tool for each task.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HashingVsEncryptionVsEncoding {
    inner: Mutex<HashMap<String, String>>,
}

impl HashingVsEncryptionVsEncoding {
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
    fn test_hashing_vs_encryption_vs_encoding() {
        let s = HashingVsEncryptionVsEncoding::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
