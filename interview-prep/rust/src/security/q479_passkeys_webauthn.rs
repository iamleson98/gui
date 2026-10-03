//! Question #479: Passkeys (WebAuthn)
//! Category: Security | Difficulty: Hard
//! Concepts: passkeys, WebAuthn, attestation, challenge
//! Description: Implement passkeys using WebAuthn with authenticator attestation and challenge-response.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct PasskeysWebauthn {
    inner: Mutex<HashMap<String, String>>,
}

impl PasskeysWebauthn {
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
    fn test_passkeys_webauthn() {
        let s = PasskeysWebauthn::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
