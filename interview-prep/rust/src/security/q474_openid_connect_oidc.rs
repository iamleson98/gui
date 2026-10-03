//! Question #474: OpenID Connect (OIDC)
//! Category: Security | Difficulty: Hard
//! Concepts: OIDC, ID token, OAuth, identity
//! Description: Layer OpenID Connect on OAuth 2.0 for authenticated identity via ID tokens.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct OpenidConnectOidc {
    inner: Mutex<HashMap<String, String>>,
}

impl OpenidConnectOidc {
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
    fn test_openid_connect_oidc() {
        let s = OpenidConnectOidc::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
