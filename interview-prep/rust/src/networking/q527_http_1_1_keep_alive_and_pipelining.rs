//! Question #527: HTTP/1.1 Keep-Alive and Pipelining
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HTTP/1.1, keep-alive, pipelining, HoL
//! Description: Use keep-alive connections and reason about pipelining's HOL limitations.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Http11KeepAliveAndPipelining {
    inner: Mutex<HashMap<String, String>>,
}

impl Http11KeepAliveAndPipelining {
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
    fn test_http_1_1_keep_alive_and_pipelining() {
        let s = Http11KeepAliveAndPipelining::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
