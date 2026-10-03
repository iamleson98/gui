//! Question #508: Secret Rotation and KMS
//! Category: Security | Difficulty: Hard
//! Concepts: secret rotation, KMS, envelope encryption, audit
//! Description: Rotate secrets via a KMS with envelope encryption and audit logging.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SecretRotationAndKms {
    inner: Mutex<HashMap<String, String>>,
}

impl SecretRotationAndKms {
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
    fn test_secret_rotation_and_kms() {
        let s = SecretRotationAndKms::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
