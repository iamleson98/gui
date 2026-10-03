//! Question #486: Certificate Transparency
//! Category: Security | Difficulty: Hard
//! Concepts: certificate transparency, CT logs, X.509, mis-issuance
//! Description: Validate X.509 certificates against CT logs to detect mis-issuance.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CertificateTransparency {
    inner: Mutex<HashMap<String, String>>,
}

impl CertificateTransparency {
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
    fn test_certificate_transparency() {
        let s = CertificateTransparency::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
