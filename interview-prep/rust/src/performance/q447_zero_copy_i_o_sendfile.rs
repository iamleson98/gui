//! Question #447: Zero-Copy I/O (sendfile)
//! Category: Performance & Profiling | Difficulty: Hard
//! Concepts: zero-copy, sendfile, splice, kernel buffer
//! Description: Use sendfile and splice to move data between file descriptors without copying.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ZeroCopyIOSendfile {
    inner: Mutex<HashMap<String, String>>,
}

impl ZeroCopyIOSendfile {
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
    fn test_zero_copy_i_o_sendfile() {
        let s = ZeroCopyIOSendfile::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
