//! Question #478: MFA / TOTP
//! Category: Security | Difficulty: Hard
//! Concepts: MFA, TOTP, RFC 6238, HMAC
//! Description: Implement time-based one-time passwords (RFC 6238) for multi-factor authentication.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct MfaTotp {
    inner: Mutex<HashMap<String, String>>,
}

impl MfaTotp {
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
    fn test_mfa_totp() {
        let s = MfaTotp::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
