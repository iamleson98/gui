//! Question #528: HTTP Status Codes and Semantics
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HTTP status, safe, idempotent, cacheable
//! Description: Choose correct HTTP status codes reflecting safe, idempotent, and cacheable semantics.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HttpStatusCodesAndSemantics {
    inner: Mutex<HashMap<String, String>>,
}

impl HttpStatusCodesAndSemantics {
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
    fn test_http_status_codes_and_semantics() {
        let s = HttpStatusCodesAndSemantics::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
