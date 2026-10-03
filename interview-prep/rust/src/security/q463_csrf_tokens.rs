//! Question #463: CSRF Tokens
//! Category: Security | Difficulty: Hard
//! Concepts: CSRF, tokens, SameSite, origin
//! Description: Prevent cross-site request forgery with anti-CSRF tokens and SameSite cookies.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct CsrfTokens {
    inner: Mutex<HashMap<String, String>>,
}

impl CsrfTokens {
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
    fn test_csrf_tokens() {
        let s = CsrfTokens::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
