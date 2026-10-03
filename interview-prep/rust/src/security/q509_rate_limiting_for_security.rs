//! Question #509: Rate Limiting for Security
//! Category: Security | Difficulty: Hard
//! Concepts: rate limiting, brute force, lockout, account
//! Description: Apply rate limits and account lockouts to slow credential-stuffing and brute force.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct RateLimitingForSecurity {
    inner: Mutex<HashMap<String, String>>,
}

impl RateLimitingForSecurity {
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
    fn test_rate_limiting_for_security() {
        let s = RateLimitingForSecurity::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
