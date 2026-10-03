//! Question #533: 0-RTT TLS
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: 0-RTT, resumption, replay, TLS 1.3
//! Description: Achieve 0-RTT resumption and reason about its replay risk for non-idempotent requests.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct 0RttTls {
    inner: Mutex<HashMap<String, String>>,
}

impl 0RttTls {
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
    fn test_0_rtt_tls() {
        let s = 0RttTls::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
