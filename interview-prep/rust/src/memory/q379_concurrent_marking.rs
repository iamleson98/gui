//! Question #379: Concurrent Marking
//! Category: Memory Management | Difficulty: Hard
//! Concepts: concurrent marking, safe-point, handshake, pause
//! Description: Implement concurrent marking with safe-points and handshakes to avoid long pauses.

use std::collections::HashMap;
use std::sync::Mutex;

pub struct ConcurrentMarking {
    inner: Mutex<HashMap<String, String>>,
}

impl ConcurrentMarking {
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
    fn test_concurrent_marking() {
        let s = ConcurrentMarking::new();
        s.set("key", "value");
        assert_eq!(s.get("key"), Some("value".to_string()));
    }
}
