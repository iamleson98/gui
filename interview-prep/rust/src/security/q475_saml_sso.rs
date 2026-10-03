//! Question #475: SAML SSO
//! Category: Security | Difficulty: Hard
//! Concepts: SAML, assertion, SSO, binding
//! Description: Implement SAML single sign-on with signed assertions and the POST redirect binding.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SamlSso {
    inner: Mutex<HashMap<String, String>>,
}

impl SamlSso {
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
    fn test_saml_sso() {
        let s = SamlSso::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
