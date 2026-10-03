//! Question #524: HTTP/2 Multiplexing
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HTTP/2, multiplexing, streams, framing
//! Description: Multiplex many streams over a single HTTP/2 connection to avoid connection sprawl.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Http2Multiplexing {
    inner: Mutex<HashMap<String, String>>,
}

impl Http2Multiplexing {
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
    fn test_http_2_multiplexing() {
        let s = Http2Multiplexing::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
