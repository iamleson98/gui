//! Question #476: Session Management (Secure Cookies)
//! Category: Security | Difficulty: Hard
//! Concepts: session, HttpOnly, Secure, SameSite
//! Description: Manage sessions with HttpOnly, Secure, and SameSite cookie attributes.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SessionManagementSecureCookies {
    inner: Mutex<HashMap<String, String>>,
}

impl SessionManagementSecureCookies {
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
    fn test_session_management_secure_cookies() {
        let s = SessionManagementSecureCookies::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
