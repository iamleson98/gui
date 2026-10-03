//! Question #451: Direct I/O vs Buffered I/O
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: direct I/O, buffered I/O, page cache, O_DIRECT
//! Description: Contrast O_DIRECT with buffered I/O and the page-cache implications.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct DirectIOVsBufferedIO {
    inner: Mutex<HashMap<String, String>>,
}

impl DirectIOVsBufferedIO {
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
    fn test_direct_i_o_vs_buffered_i_o() {
        let s = DirectIOVsBufferedIO::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
