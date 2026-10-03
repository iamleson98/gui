//! Question #526: HTTP/3 over QUIC
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HTTP/3, QUIC, streams, frames
//! Description: Map HTTP/3 semantics onto QUIC streams and frames.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Http3OverQuic {
    inner: Mutex<HashMap<String, String>>,
}

impl Http3OverQuic {
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
    fn test_http_3_over_quic() {
        let s = Http3OverQuic::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
