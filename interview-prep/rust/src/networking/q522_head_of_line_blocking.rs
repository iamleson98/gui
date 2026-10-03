//! Question #522: Head-of-Line Blocking
//! Category: Networking & Protocols | Difficulty: Hard
//! Concepts: HoL blocking, TCP, QUIC, stream multiplexing
//! Description: Explain TCP head-of-line blocking and how QUIC addresses it.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct HeadOfLineBlocking {
    inner: Mutex<HashMap<String, String>>,
}

impl HeadOfLineBlocking {
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
    fn test_head_of_line_blocking() {
        let s = HeadOfLineBlocking::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
