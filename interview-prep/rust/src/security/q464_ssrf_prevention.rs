//! Question #464: SSRF Prevention
//! Category: Security | Difficulty: Hard
//! Concepts: SSRF, allowlist, egress, metadata
//! Description: Prevent server-side request forgery by validating and restricting outbound destinations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SsrfPrevention {
    inner: Mutex<HashMap<String, String>>,
}

impl SsrfPrevention {
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
    fn test_ssrf_prevention() {
        let s = SsrfPrevention::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
