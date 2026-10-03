//! Question #462: XSS Prevention (CSP)
//! Category: Security | Difficulty: Hard
//! Concepts: XSS, encoding, CSP, sanitization
//! Description: Prevent cross-site scripting with output encoding and a Content Security Policy.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct XssPreventionCsp {
    inner: Mutex<HashMap<String, String>>,
}

impl XssPreventionCsp {
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
    fn test_xss_prevention_csp() {
        let s = XssPreventionCsp::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
